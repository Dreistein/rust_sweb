add_project_library(common_rust)

set(CARGO_CMD cargo build --release)
set(TARGET_DIR "release")

set(RUST_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/librust_parts.a")
set(RUST_TARGET ${LIBRARY_OUTPUT_PATH})

add_custom_target(rust_parts ALL
    COMMENT "Compiling rust module"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
    COMMAND cp "${RUST_SO}" "${RUST_TARGET}"
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(rust_parts PROPERTIES LOCATION ${RUST_TARGET})

set(ENV{LIBRARY_NAMES} "$ENV{LIBRARY_NAMES};rust_parts")