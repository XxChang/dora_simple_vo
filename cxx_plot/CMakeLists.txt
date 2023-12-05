option(ENABLE_LTO "Enable cross lanuage linking time optimization" OFF)
if(ENABLE_LTO)
    include(CheckIPOSupported)
    check_ipo_supported(RESULT supported OUTPUT error)
    if(supported)
        message(STATUS "IPO / LTO enabled")
        set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)
        add_link_options(-fuse-ld=lld)
    else()
        message(STATUS "IPO / LTO not supported: <${error}>")
    endif()
endif()

include_directories(${CMAKE_BINARY_DIR}/cxx_plot/rust_part)

add_subdirectory(rust_part)

find_package(Arrow CONFIG REQUIRED)

find_package(OpenCV REQUIRED)
include_directories(${OpenCV_INCLUDE_DIRS})

add_executable(cxx_plot main.cc)
target_link_libraries(cxx_plot PRIVATE "$<IF:$<BOOL:${ARROW_BUILD_STATIC}>,Arrow::arrow_static,Arrow::arrow_shared>" rust_part  ${OpenCV_LIBS})

install(TARGETS cxx_plot DESTINATION ${CMAKE_CURRENT_SOURCE_DIR}/bin)
