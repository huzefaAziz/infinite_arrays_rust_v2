cmake_minimum_required(VERSION 3.15)
project(infinite-arrays VERSION 0.1.0 LANGUAGES Rust)

# Set Rust toolchain
set(CMAKE_RUST_STD_VERSION "2021")

# Find Rust
find_program(CARGO cargo REQUIRED)
find_program(RUSTC rustc REQUIRED)

# Set build type
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif()

# Rust build configuration
if(CMAKE_BUILD_TYPE STREQUAL "Release")
    set(CARGO_BUILD_TYPE "--release")
    set(CARGO_TARGET_DIR "${CMAKE_BINARY_DIR}/target/release")
else()
    set(CARGO_BUILD_TYPE "")
    set(CARGO_TARGET_DIR "${CMAKE_BINARY_DIR}/target/debug")
endif()

# Determine target triple
if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    if(CMAKE_SYSTEM_PROCESSOR MATCHES "AMD64|x86_64")
        set(RUST_TARGET "x86_64-pc-windows-msvc")
    elseif(CMAKE_SYSTEM_PROCESSOR MATCHES "i[3-6]86")
        set(RUST_TARGET "i686-pc-windows-msvc")
    else()
        set(RUST_TARGET "x86_64-pc-windows-msvc")
    endif()
elseif(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    if(CMAKE_SYSTEM_PROCESSOR MATCHES "AMD64|x86_64")
        set(RUST_TARGET "x86_64-unknown-linux-gnu")
    elseif(CMAKE_SYSTEM_PROCESSOR MATCHES "aarch64|ARM64")
        set(RUST_TARGET "aarch64-unknown-linux-gnu")
    else()
        set(RUST_TARGET "x86_64-unknown-linux-gnu")
    endif()
elseif(CMAKE_SYSTEM_NAME STREQUAL "Darwin")
    if(CMAKE_SYSTEM_PROCESSOR MATCHES "AMD64|x86_64")
        set(RUST_TARGET "x86_64-apple-darwin")
    elseif(CMAKE_SYSTEM_PROCESSOR MATCHES "aarch64|ARM64")
        set(RUST_TARGET "aarch64-apple-darwin")
    else()
        set(RUST_TARGET "x86_64-apple-darwin")
    endif()
else()
    set(RUST_TARGET "")
endif()

# Build Rust library
add_custom_command(
    OUTPUT ${CARGO_TARGET_DIR}/libinfinite_arrays${CMAKE_SHARED_LIBRARY_SUFFIX}
           ${CARGO_TARGET_DIR}/libinfinite_arrays${CMAKE_STATIC_LIBRARY_SUFFIX}
    COMMAND ${CARGO} build ${CARGO_BUILD_TYPE} --target ${RUST_TARGET}
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Building Rust library infinite-arrays"
    VERBATIM
)

# Create library target
add_library(infinite_arrays SHARED IMPORTED)
set_target_properties(infinite_arrays PROPERTIES
    IMPORTED_LOCATION ${CARGO_TARGET_DIR}/libinfinite_arrays${CMAKE_SHARED_LIBRARY_SUFFIX}
)

# Add static library option
add_library(infinite_arrays_static STATIC IMPORTED)
set_target_properties(infinite_arrays_static PROPERTIES
    IMPORTED_LOCATION ${CARGO_TARGET_DIR}/libinfinite_arrays${CMAKE_STATIC_LIBRARY_SUFFIX}
)

# Include directories (for C/C++ bindings if needed)
include_directories(${CMAKE_SOURCE_DIR}/include)

# Example executable
add_executable(example_basic examples/basic_usage.rs)
target_link_libraries(example_basic infinite_arrays)

add_executable(example_iqr examples/iqr_example.rs)
target_link_libraries(example_iqr infinite_arrays)

# Install rules
install(TARGETS infinite_arrays infinite_arrays_static
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
)

# Custom target for cargo test
add_custom_target(cargo_test
    COMMAND ${CARGO} test
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Running Rust tests"
)

# Custom target for cargo doc
add_custom_target(cargo_doc
    COMMAND ${CARGO} doc --no-deps
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Generating Rust documentation"
)

# Print configuration
message(STATUS "Rust compiler: ${RUSTC}")
message(STATUS "Cargo: ${CARGO}")
message(STATUS "Rust target: ${RUST_TARGET}")
message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")
message(STATUS "Target directory: ${CARGO_TARGET_DIR}")

