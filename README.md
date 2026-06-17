Stub generation

```sh
maturin generate-stubs --out python/cr_core
```

- Strictly separating the core rust module from the bindings requires a lot of boilerplate code
- This would be much better, when combining them, but I think that's not an option

- Python package structure is defined in python/cr_core (in editable mode, the .so is installed into this directory instead of the .venv)
