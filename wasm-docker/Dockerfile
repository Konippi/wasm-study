FROM scratch

COPY ./target/wasm32-wasip1/debug/wasm-docker.wasm /wasm-docker.wasm

ENTRYPOINT [ "/wasm-docker.wasm" ]