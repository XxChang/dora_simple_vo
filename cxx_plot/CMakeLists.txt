find_package(Arrow REQUIRED)

add_executable(cxx_plot main.cc ${node_bridge})
add_dependencies(cxx_plot Dora_cxx)
target_include_directories(cxx_plot PRIVATE ${dora_cxx_include_dir} ${dora_c_include_dir})
target_link_libraries(cxx_plot PRIVATE Arrow::arrow_shared dora_node_api_cxx)

install(TARGETS cxx_plot DESTINATION ${CMAKE_CURRENT_SOURCE_DIR}/bin)
