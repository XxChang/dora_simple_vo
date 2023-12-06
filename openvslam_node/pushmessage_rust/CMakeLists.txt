if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build --verbose)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release --verbose)
    set(TARGET_DIR "release")
endif ()

set(RUST_PART_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libpushmessage_rust.a")

set(RUST_PART_CXX "${CMAKE_CURRENT_BINARY_DIR}/pushmessage_rust.cpp")
add_library(pushmessage_rust STATIC ${RUST_PART_CXX})
add_custom_command(
    OUTPUT ${RUST_PART_CXX}
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} RUSTFLAGS="${RUST_FLAGS}" ${CARGO_CMD}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/pushmessage_rust/src/lib.rs.cc ${RUST_PART_CXX}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/pushmessage_rust/src/lib.rs.h ${CMAKE_CURRENT_BINARY_DIR}/pushmessage_rust.h
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

target_link_libraries(pushmessage_rust pthread dl ${RUST_PART_LIB})
