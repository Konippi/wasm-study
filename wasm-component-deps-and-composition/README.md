# Wasm Component Dependencies and Composition

This is a simple example for using the dependent Wasm components and composing them together.

## Steps

1. Compose the Wasm components

```bash
> cd caller-with-composed-component
> wac plug target/wasm32-unknown-unknown/debug/say.wasm \
    --plug target/wasm32-unknown-unknown/debug/greet.wasm \
    -o composed-greet.wasm
```

2. Run the composed Wasm component

```bash
> cargo run -- composed-greet.wasm
```