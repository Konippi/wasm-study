# Wasm Component

This is a simple example for using exported Wasm Library Component.

## Steps

1. Build the Wasm Library Component using `cargo-component` command.

```bash
> cargo component build --target wasm32-unknown-unknown
```

2. Run the Wasm binary.

- `wasm-lib-component-caller`
    
    ```bash
    > cd wasm-lib-component-caller
    > cargo run ../target/wasm32-unknown-unknown/debug/wasm_lib_component.wasm
    ```

- `wasm-lib-component-caller-with-wasmtime-bindgen`

    ```bash
    > cd wasm-lib-component-caller-with-wasmtime-bindgen
    > cargo run ../target/wasm32-unknown-unknown/debug/wasm_lib_component.wasm
    ```
