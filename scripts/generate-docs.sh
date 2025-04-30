echo "🧩 Copying files needed for typedoc to pkg..."
cp tsconfig pkg/tsconfig.json
cp typedoc pkg/typedoc.json

echo "🧩 Installing typedoc..."
cd pkg
pnpm i -D typedoc

echo "🧩 Calling annotate internals script..."
cd ..
node ./scripts/annotate-internals.js

echo "🧩 Running typedoc..."
cd pkg
pnpm exec typedoc ./sticknodes_js.d.ts --excludeInternal --sort enum-value-ascending

cd ..

echo "✅ Docs generated"