#define USE_RERUN 0

#include <iostream>
#include <stella_vslam/config.h>
#include <stella_vslam/system.h>
#include <algorithm>
#include <numeric>
#include <stella_vslam/publish/frame_publisher.h>

#if USE_RERUN
#else
#include <pangolin_viewer/viewer.h>
#endif

#include <stella_vslam/util/yaml.h>

#include "pushmessage_rust.h"

int main(int argc, char **argv) {
    auto vocab_file_path = std::string(argv[1]);
    auto config_file_path = std::string(argv[2]);

    std::cout << "vocab_file_path: " << vocab_file_path << std::endl;
    std::cout << "config_file_path: " << config_file_path << std::endl;

    std::vector<double> track_times;

    auto cfg = std::make_shared<stella_vslam::config>(config_file_path);

    auto slam = std::make_shared<stella_vslam::system>(cfg, vocab_file_path);

    slam->startup(true);

#if USE_RERUN
    bool vslam_closed = false;
    auto frame_publisher = slam->get_frame_publisher();
#else
    std::shared_ptr<pangolin_viewer::viewer> viewer;
    viewer = std::make_shared<pangolin_viewer::viewer>(
        stella_vslam::util::yaml_optional_ref(cfg->yaml_node_, "PangolinViewer"),
        slam,
        slam->get_frame_publisher(),
        slam->get_map_publisher());
#endif

    auto dora_node = init_dora_node();

    std::thread thread([&]() {
#if USE_RERUN

        while (!vslam_closed)
        {
            float fps = cfg->yaml_node_["fps"].as<float>(30.0);
            float interval_time = 1000.0/fps;
            // 3 channel BGR
            
            auto camera = slam->get_camera();
            cv::Mat frame = frame_publisher->draw_frame();

            rust::cxxbridge1::Slice<const uint8_t> data{frame.data, frame.total() * frame.elemSize()};
            auto result = put_frame(dora_node.send_output, "color_frame", frame.cols, frame.rows, data);
            std::cout << result.error.c_str() << std::endl;
            std::this_thread::sleep_for(std::chrono::milliseconds(int64_t(interval_time)));
        }
#else
        viewer->run();
#endif
    });

    for (;;) {
        auto event = next_event(dora_node.events);
        auto ty = event_type(event);

        if (ty == DoraEventType::AllInputsClosed) {
            break;
        }
        else if (ty == DoraEventType::Input) {
            auto input = get_pic_from_event(std::move(event));

            cv::Mat l_image{
                static_cast<int>(input.height_l), 
                static_cast<int>(input.width_l),
                CV_8UC1,
                input.raw_data_l.data()
            };

            cv::Mat r_image{
                static_cast<int>(input.height_r), 
                static_cast<int>(input.width_r),
                CV_8UC1,
                input.raw_data_r.data()
            };

            const auto tp_1 = std::chrono::steady_clock::now();

            std::chrono::system_clock::time_point now = std::chrono::system_clock::now();
            double timestamp = std::chrono::duration_cast<std::chrono::duration<double>>(now.time_since_epoch()).count();

            slam->feed_stereo_frame(l_image, r_image, timestamp);

            const auto tp_2 = std::chrono::steady_clock::now();

            const auto track_time = std::chrono::duration_cast<std::chrono::duration<double>>(tp_2 - tp_1).count();
            track_times.push_back(track_time);
        }
        else {
            std::cerr << "Unknown event type " << static_cast<int>(ty) << std::endl;
            break;
        }
    }

#if USE_RERUN
    vslam_closed = true;
#endif

    thread.join();

    // wait until the loop BA is finished
    while (slam->loop_BA_is_running()) {
        std::this_thread::sleep_for(std::chrono::microseconds(5000));
    }

    slam->shutdown();

    std::sort(track_times.begin(), track_times.end());
    const auto total_track_time = std::accumulate(track_times.begin(), track_times.end(), 0.0);
    std::cout << "median tracking time: " << track_times.at(track_times.size() / 2) << "[s]" << std::endl;
    std::cout << "mean tracking time: " << total_track_time / track_times.size() << "[s]" << std::endl;

    return 0;
}
