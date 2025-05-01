echo "🧩 Copying files needed for typedoc to pkg..."
cp tsconfig pkg/tsconfig.json
cp typedoc pkg/typedoc.json
cp README.docs.md pkg/README.docs.md

echo "🧩 Copying files needed for vitepress to pkg/vpdocs..."
cp -r vpdocs pkg/vpdocs

echo "🧩 Installing typedoc..."
cd pkg
pnpm i -D typedoc

echo "🧩 Installing vitepress..."
pnpm add -D vitepress

echo "🧩 Calling annotate internals script..."
cd ..
node ./scripts/annotate-internals.js

echo "🧩 Running typedoc..."
cd pkg
pnpm exec typedoc ./sticknodes_js.d.ts --excludeInternal --sort enum-value-ascending

echo "✅ Docs generated"

echo "🧩 Running vitepress..."
pnpm run docs:build

cd ..

echo "🧩 Moving typedoc docs to vitepress..."
mv pkg/docs pkg/vpdocs/.vitepress/dist/docs

echo "✅ vitepress set up"