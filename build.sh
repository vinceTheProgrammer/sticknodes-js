#!/bin/bash

# build.sh
wasm-pack build --target bundler --dev
node ./scripts/postbuild.js
