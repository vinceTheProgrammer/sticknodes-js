#!/bin/bash

# build.sh
cargo test
./scripts/build-all.sh
node ./scripts/postbuild.js
node scripts/update-readme-version.js
./scripts/generate-docs.sh