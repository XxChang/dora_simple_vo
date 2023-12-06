find_package(stella_vslam REQUIRED)
find_package(yaml-cpp REQUIRED)
find_package(Eigen3 REQUIRED)

include_directories(
    ${stella_vslam_INCLUDE_DIRS} 
    ${stella_vslam_INCLUDE_DIRS}/stella_vslam/3rd/json/include
    ${stella_vslam_INCLUDE_DIRS}/stella_vslam/3rd/spdlog/include
)

include_directories(${CMAKE_BINARY_DIR}/openvslam_node/pushmessage_rust)

add_subdirectory(pushmessage_rust)

find_package(OpenCV 3.3 REQUIRED)
include_directories(${OpenCV_INCLUDE_DIRS})

find_package(pangolin_viewer REQUIRED)
include_directories(${pangolin_viewer_INCLUDE_DIRS})

find_package(Pangolin REQUIRED)

add_executable(openvslam_node main.cc)
target_link_libraries(openvslam_node PRIVATE 
            pushmessage_rust 
            ${stella_vslam_LIBS} 
            Eigen3::Eigen 
            ${OpenCV_LIBS}
            pangolin_viewer::pangolin_viewer
            ${Pangolin_LIBRARIES})

install(TARGETS openvslam_node DESTINATION ${CMAKE_CURRENT_SOURCE_DIR}/bin)

