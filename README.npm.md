# sticknodes-js

This is the JavaScript/TypeScript bindings for [sticknodes-rs](https://github.com/vinceTheProgrammer/sticknodes-rs). It allows manipulation of Stick Nodes Stickfigures from within a web or Node.js environment.

## Installation

To install via NPM:
```bash
npm install sticknodes-js
```

## Usage
### In the Browser (Web)
```html
<script type="module">
    import init, * as sn from "https://cdn.jsdelivr.net/npm/sticknodes-js@VERSION/sticknodes_js_web.js";
    init().then(() => {
        // Example usage
        const stickfigure = new sn.Stickfigure();

        // Reference docs (or source code if docs don't exist yet) for other usage
    });
</script>
```

### In Node.js
```js
import init from "sticknodes-js";
init().then(module => {
  // Your code here
});
```

### In Bundlers (e.g., Webpack, Rollup, Parcel)
```js
import init from "sticknodes-js";
init().then(module => {
  // Your code here
});
```

## TypeScript Definitions
The library includes TypeScript definitions to ensure smooth integration with TypeScript projects.

## License
MIT License - See [LICENSE](./LICENSE)