# WAT to Wasm binary

The function that adds two numbers is defined in the `add.wat` file. <br>
The `add.wat` file is compiled to a Wasm binary file using the `wat2wasm` command.

## Steps

1. Convert WAT to Wasm binary using the `wat2wasm` command.

```bash
> wat2wasm add.wat -o add.wasm
```

2. Run the Wasm binary using the `wasmtime` command.

```bash
> wasmtime --invoke add add.wasm 1 2
```
