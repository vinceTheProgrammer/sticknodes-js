#!/bin/bash

# build.sh
cargo test
wasm-pack build --target bundler --dev
node ./scripts/postbuild.js
