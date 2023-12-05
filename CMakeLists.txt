cmake_minimum_required(VERSION 3.21)
project(dora-simple-vo LANGUAGES C CXX)

# set(VCPKG_ROOT "" CACHE FILEPATH "Path to the root of vcpkg")
# set(CMAKE_TOOLCHAIN_FILE ${VCPKG_ROOT})

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS "-fPIC")

# include(DoraTargets.cmake)

# link_directories(${dora_link_dirs})

add_subdirectory(cxx_plot)
