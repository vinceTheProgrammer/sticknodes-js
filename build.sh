#!/bin/bash

# build.sh
cargo test
./scripts/build-all.sh
node ./scripts/postbuild.js
