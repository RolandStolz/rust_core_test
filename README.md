Stub generation

```sh
maturin generate-stubs --out python/cr_core
```

## C++ bindings (cxx)

The `cpp` feature exposes the core structs (`Point`, `State`, `Lanelet`) to C++ as
`cxx` shared structs — usable by value on both sides. Bridge lives in
`src/bindings/cpp.rs`; `build.rs` compiles the cxx glue + `cpp/demo.cc` into the
staticlib. Mirrors the `python` feature pattern.

Verify the bridge (Rust calls a C++ consumer of the structs):

```sh
cargo test --features cpp
```

Standalone C++ app (Corrosion builds + links the Rust core):

```sh
cmake -S . -B build-cpp -DCMAKE_BUILD_TYPE=Release
cmake --build build-cpp
./build-cpp/cpp_demo
```

- Strictly separating the core rust module from the bindings requires a lot of boilerplate code
- This would be much better, when combining them, but I think that's not an option

- Python package structure is defined in python/cr_core (in editable mode, the .so is installed into this directory instead of the .venv)
