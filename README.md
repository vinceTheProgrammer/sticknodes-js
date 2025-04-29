# sticknodes-rs wasm-bindgen bindings for sticknodes-js

Wasm-bindgen JavaScript/TypeScript bindings for `sticknodes-rs`

## 🧩 JavaScript/TypeScript Bindings

JS bindings are published to npm:  
👉 [sticknodes-js on npm](https://www.npmjs.com/package/sticknodes-js)

📖 See [`README.npm.md`](./README.npm.md) for full usage instructions.

## Setting up the Rust environment
1. Install Rust: https://www.rust-lang.org/tools/install
2. Install `wasm-pack` for building the WebAssembly bindings:
   ```bash
   cargo install wasm-pack
   ```
## Building WebAssembly Bindings
To generate the JavaScript bindings for all targets, make sure you have jq installed, and then run:
```bash
./build.sh
```

## License
MIT License - See [LICENSE](./LICENSE)