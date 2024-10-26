# `grep` command using Wasm

This is a simple `grep` command implemented in Wasm.

## Steps

1. Build the component using the `cargo-component` command.

```bash
> cargo component build
```

2. Run the Wasm binary using the `wasmtime` command.

```bash
> wasmtime --dir . target/wasm32-wasip1/debug/wasm-grep.wasm file src/main.rs
```
