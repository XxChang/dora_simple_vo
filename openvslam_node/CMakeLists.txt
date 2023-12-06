find_package(stella_vslam REQUIRED)
find_package(yaml-cpp REQUIRED)
find_package(Eigen3 REQUIRED)

include_directories(
    ${stella_vslam_INCLUDE_DIRS} 
    ${stella_vslam_INCLUDE_DIRS}/stella_vslam/3rd/json/include
)

include_directories(${CMAKE_BINARY_DIR}/openvslam_node/pushmessage_rust)

add_subdirectory(pushmessage_rust)

find_package(OpenCV 3.3 REQUIRED)
include_directories(${OpenCV_INCLUDE_DIRS})

add_executable(openvslam_node main.cc)
target_link_libraries(openvslam_node PRIVATE pushmessage_rust ${stella_vslam_LIBS} Eigen3::Eigen ${OpenCV_LIBS})

install(TARGETS openvslam_node DESTINATION ${CMAKE_CURRENT_SOURCE_DIR}/bin)

