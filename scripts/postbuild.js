const fs = require("fs");
const path = require("path");

const PKG_PATH = path.resolve(__dirname, "../pkg", "sticknodes_js.d.ts");
const NODE_OPTIONS_PATH = path.resolve(__dirname, "../bindings", "NodeOptions.ts");

const nodeOptionsContent = fs.readFileSync(NODE_OPTIONS_PATH, "utf-8");
const nodeOptionsMatch = nodeOptionsContent.match(/export type NodeOptions\s*=\s*{[^}]+}/s);

if (!nodeOptionsMatch) {
  console.error("❌ Could not find NodeOptions type in NodeOptions.ts");
  process.exit(1);
}

const nodeOptionsType = nodeOptionsMatch[0];
let dtsContent = fs.readFileSync(PKG_PATH, "utf-8");

if (!dtsContent.includes("export type NodeOptions")) {
  dtsContent = `${nodeOptionsType}\n\n${dtsContent}`;
}

dtsContent = dtsContent.replace(
  /add_child\s*\(options:\s*any\):\s*Node;/,
  "add_child(options: NodeOptions): Node;"
);

fs.writeFileSync(PKG_PATH, dtsContent);
console.log("✅ Updated sticknodes_js.d.ts with NodeOptions type.");
