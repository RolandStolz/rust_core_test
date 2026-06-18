cmake_minimum_required(VERSION 3.16)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(cpp_consumer LANGUAGES CXX)

# --- Generate the cxx headers from the Rust core (rust_core_test) -------------
# Building cr_core with the `cpp` feature runs its build.rs (cxx_build), which emits
# the shared-struct headers to <core>/target/cxxbridge. We only need the headers
# (the structs are plain data); nothing Rust is linked.
get_filename_component(CORE_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../.." ABSOLUTE)
find_program(CARGO cargo REQUIRED)
message(STATUS "cpp_consumer: generating cxx headers via cargo build --features cpp")
execute_process(
    COMMAND "${CARGO}" build --features cpp --manifest-path "${CORE_ROOT}/Cargo.toml"
    RESULT_VARIABLE _cargo_rc
)
if(NOT _cargo_rc EQUAL 0)
    message(FATAL_ERROR "cargo build --features cpp failed (${_cargo_rc})")
endif()
set(CXXBRIDGE_DIR "${CORE_ROOT}/target/cxxbridge")

# --- nanobind ----------------------------------------------------------------
find_package(Python COMPONENTS Interpreter Development.Module REQUIRED)
execute_process(
    COMMAND "${Python_EXECUTABLE}" -m nanobind --cmake_dir
    OUTPUT_VARIABLE nanobind_DIR
    OUTPUT_STRIP_TRAILING_WHITESPACE
    COMMAND_ERROR_IS_FATAL ANY
)
find_package(nanobind CONFIG REQUIRED)

# --- Core logic layer: plain functions over cr_core structs, no Python ----------
# CXXBRIDGE_DIR: cr_core/src/bindings/cpp.rs.h + rust/cxx.h.
# CORE_ROOT/cpp/include: cr_core/demo.h, which the generated header #includes.
add_library(cpp_consumer_logic STATIC src/logic.cpp)
target_compile_features(cpp_consumer_logic PUBLIC cxx_std_17)
set_property(TARGET cpp_consumer_logic PROPERTY POSITION_INDEPENDENT_CODE ON)
target_include_directories(cpp_consumer_logic PUBLIC "${CXXBRIDGE_DIR}" "${CORE_ROOT}/cpp/include")

# --- nanobind binding layer: capsule pointer extraction, delegates to the logic --
nanobind_add_module(cpp_consumer MODULE src/bindings.cpp)
target_link_libraries(cpp_consumer PRIVATE cpp_consumer_logic)

# --- Type stub (.pyi) generated from the compiled module ------------------------
nanobind_add_stub(
    cpp_consumer_stub
    MODULE cpp_consumer
    OUTPUT cpp_consumer.pyi
    PYTHON_PATH $<TARGET_FILE_DIR:cpp_consumer>
    DEPENDS cpp_consumer
)

if(SKBUILD)
    install(TARGETS cpp_consumer LIBRARY DESTINATION .)
    install(FILES "${CMAKE_CURRENT_BINARY_DIR}/cpp_consumer.pyi" DESTINATION .)
endif()
