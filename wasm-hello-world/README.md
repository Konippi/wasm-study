# Hello World in Wasm

The source code for printing "Hello, World!" in Wasm.

## Steps

1. Build the component using the `cargo-component` command.

```bash
> cargo component build
```

2. Run the Wasm binary using the `wasmtime` command.

```bash
> wasmtime target/wasm32-wasip1/debug/wasm-hello-world.wasm
```
