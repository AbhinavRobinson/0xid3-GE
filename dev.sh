# RUNS Cargo wasm build with hot-reload and page server
cargo watch -x 'build --example game --target wasm32-unknown-unknown' | devserver
