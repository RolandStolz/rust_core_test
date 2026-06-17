cmake_minimum_required(VERSION 3.22)
project(cr_core_cpp_demo LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Corrosion drives `cargo build` and imports the crate as a CMake target,
# resolving the native system libraries the Rust staticlib needs to link.
include(FetchContent)
FetchContent_Declare(
  Corrosion
  GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
  GIT_TAG v0.5.1
)
FetchContent_MakeAvailable(Corrosion)

# Build cr_core with the `cpp` feature -> staticlib.
# Its build.rs compiles the cxx glue + cpp/demo.cc into the archive and emits
# the generated headers under <cargo-target-dir>/cxxbridge.
corrosion_import_crate(
  MANIFEST_PATH Cargo.toml
  CRATE_TYPES staticlib
  FEATURES cpp
)

# Locate the cxx-generated headers (rust/cxx.h, cr_core/src/bindings/cpp.rs.h).
# Corrosion exposes the cargo target dir via this cache variable.
set(CXXBRIDGE_DIR "${CMAKE_SOURCE_DIR}/target/cxxbridge")

add_executable(cpp_demo examples/main.cpp)
target_link_libraries(cpp_demo PRIVATE cr_core)
target_include_directories(cpp_demo PRIVATE
  "${CMAKE_SOURCE_DIR}/cpp/include"
  "${CXXBRIDGE_DIR}"
)
