#include "rust_part.h"
#include <iostream>
#include <opencv4/opencv2/opencv.hpp>

int main(int argc, char** argv) {
    auto dora_node = init_dora_node();
    cv::namedWindow("image", cv::WINDOW_AUTOSIZE);
    cv::Mat image;

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

            cv::vconcat(r_image.clone(), l_image, image);
            cv::imshow("image", image);
            if (cv::waitKey(1) > 0)
                break;
        }
        else {
            std::cerr << "Unknown event type " << static_cast<int>(ty) << std::endl;
            break;
        }
    } 

    cv::destroyAllWindows();
    return 0;
}
