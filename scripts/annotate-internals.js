const fs = require("fs");
const path = require("path");

const dtsPath = path.resolve(__dirname, "../pkg", "sticknodes_js.d.ts");
let content = fs.readFileSync(dtsPath, "utf8");

// Match 'export function main(): void;' not already preceded by @internal
const pattern = /^(?!.*@internal)[ \t]*export function main\(\): void;/m;

if (pattern.test(content)) {
  content = content.replace(
    pattern,
    `/** @internal */\nexport function main(): void;`
  );
  fs.writeFileSync(dtsPath, content, "utf8");
  console.log("Annotated @internal above 'main()' function.");
} else {
  console.log("No matching 'main' export found or already annotated.");
}
