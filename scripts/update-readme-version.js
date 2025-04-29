import fs from 'fs';
import pkg from '../pkg/package.json' with { type: "json" };

const version = pkg.version;
const readmePath = './pkg/README.md';
let readme = fs.readFileSync(readmePath, 'utf8');

readme = readme.replace(/@VERSION/g, `@${version}`);
fs.writeFileSync(readmePath, readme);

console.log(`Updated README import URLs to version ${version}`);