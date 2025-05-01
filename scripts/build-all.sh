#!/usr/bin/env bash

set -e

LIB_NAME="sticknodes_js"

echo "🔨 Cleaning previous builds..."
rm -rf pkg pkg-bundler pkg-nodejs pkg-web

echo "📦 Building bundler target..."
wasm-pack build --target bundler --out-dir pkg-bundler

echo "📦 Building nodejs target..."
wasm-bindgen --target experimental-nodejs-module ./target/wasm32-unknown-unknown/release/sticknodes_js.wasm --out-dir pkg-nodejs --out-name sticknodes_js_nodejs

echo "📦 Building web target..."
wasm-pack build --target web --out-dir pkg-web --out-name sticknodes_js_web

echo "📂 Creating merged pkg/ directory..."
mkdir pkg

echo "🧩 Copying bundler build..."
cp pkg-bundler/* pkg/

echo "🧩 Copying nodejs build with renamed files..."
cp pkg-nodejs/${LIB_NAME}_nodejs.js pkg/${LIB_NAME}_nodejs.js
cp pkg-nodejs/${LIB_NAME}_nodejs_bg.js pkg/${LIB_NAME}_nodejs_bg.js
cp pkg-nodejs/${LIB_NAME}_nodejs_bg.wasm pkg/${LIB_NAME}_nodejs_bg.wasm

echo "🧩 Copying web build with renamed files..."
cp pkg-web/${LIB_NAME}_web.js pkg/${LIB_NAME}_web.js
cp pkg-web/${LIB_NAME}_web_bg.wasm pkg/${LIB_NAME}_web_bg.wasm

echo "📝 Rewriting package.json..."

# Copy your existing root-level package.json as a base
cp pkg-bundler/package.json pkg/package.json

# Use jq to overwrite build-specific fields
jq --arg lib "$LIB_NAME" '
  .main = "./\($lib)_nodejs.js" |
  .types = "./\($lib).d.ts" |
  .exports = {
    "import": {
      "node": "./\($lib)_nodejs.js",
      "default": "./\($lib).js"
    },
    "require": "./\($lib)_nodejs.js",
    "web": "./\($lib)_web.js"
  } |
  .files = [
    "*.js",
    "*.wasm",
    "*.d.ts"
  ] |
  .scripts = {
    "docs:dev": "vitepress dev vpdocs",
    "docs:build": "vitepress build vpdocs",
    "docs:preview": "vitepress preview vpdocs"
  }
' pkg/package.json > pkg/package.tmp.json && mv pkg/package.tmp.json pkg/package.json

echo "✅ package.json merged and updated."

echo "📘 Copying JS README into pkg..."
cp README.npm.md pkg/README.md
echo "✅ README copied into pkg."