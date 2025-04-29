const fs = require("fs");
const path = require("path");

const PKG_PATH = path.resolve(__dirname, "../pkg", "sticknodes_js.d.ts");
const NODE_OPTIONS_PATH = path.resolve(__dirname, "../bindings", "NodeOptions.ts");
const POLYFILL_OPTIONS_PATH = path.resolve(__dirname, "../bindings", "PolyfillOptions.ts");

const nodeOptionsContent = fs.readFileSync(NODE_OPTIONS_PATH, "utf-8");
const nodeOptionsMatch = nodeOptionsContent.match(/export type NodeOptions\s*=\s*{[^}]+}/s);

const polyfillOptionsContent = fs.readFileSync(POLYFILL_OPTIONS_PATH, "utf-8");
const polyfillOptionsMatch = polyfillOptionsContent.match(/export type PolyfillOptions\s*=\s*{[^}]+}/s);

if (!nodeOptionsMatch) {
  console.error("❌ Could not find NodeOptions type in NodeOptions.ts");
  process.exit(1);
}

if (!polyfillOptionsMatch) {
  console.error("❌ Could not find PolyfillOptions type in PolyfillOptions.ts");
  process.exit(1);
}


let dtsContent = fs.readFileSync(PKG_PATH, "utf-8");

const nodeOptionsType = nodeOptionsMatch[0];
if (!dtsContent.includes("export type NodeOptions")) {
  dtsContent = `${nodeOptionsType}\n\n${dtsContent}`;
}

const polyfillOptionsType = polyfillOptionsMatch[0];
if (!dtsContent.includes("export type PolyfillOptions")) {
  dtsContent = `${polyfillOptionsType}\n\n${dtsContent}`;
}

dtsContent = dtsContent.replace(
  /add_child\s*\(options:\s*any\):\s*Node;/,
  "add_child(options: NodeOptions): Node;"
);

dtsContent = dtsContent.replace(
  /add_sibling\s*\(options:\s*any\):\s*Node;/,
  "add_sibling(options: NodeOptions): Node;"
);

dtsContent = dtsContent.replace(
  /add_polyfill\s*\(options:\s*any\):\s*Node;/,
  "add_polyfill(options: PolyfillOptions): Node;"
);

dtsContent = dtsContent.replace(
  /get_node_options\s*\(\)\s*:\s*any\s*;/,
  "get_node_options(): NodeOptions;"
);

fs.writeFileSync(PKG_PATH, dtsContent);
console.log("✅ Updated sticknodes_js.d.ts with NodeOptions type.");
