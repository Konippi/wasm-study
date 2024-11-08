# Wasm in Docker

This is a simple example of how to run a WebAssembly module in a Docker container.

## Steps

1. Build the Docker image:

```bash
> docker buildx build --platform wasi/wasm -t wasm-docker .
```

2. Run the Docker container:

```bash
> docker run --rm --runtime=io.containerd.wasmtime.v1 --platform=wasi/wasm wasm-docker
```
