cmake_minimum_required(VERSION 3.21)
project(dora-simple-vo LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS "-fPIC")

include(DoraTargets.cmake)

link_directories(${dora_link_dirs})

add_subdirectory(cxx_plot)
