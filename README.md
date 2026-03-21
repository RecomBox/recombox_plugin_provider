## Installation
Make sure you have the following installed:
- [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [NodeJS](https://nodejs.org/)
- [Bun](https://bun.com/) or [NPM](https://www.npmjs.com/) or whatever runtime you want to use


## Making Plugins
Clone this repository
```bash
git clone https://github.com/RecomBox/recombox_plugin_provider
cd recombox_plugin_provider
```

Go to `plugin_template` directory
```bash
cd plugins/plugin_template
```
- You can copy the template and rename the directory to your pluign name.

- Install dependencies:
```bash
# Or use any runtime you want
bun install
```

**[Required]** export functions inside `plugin.ts`:
```typescript
export function select_source(input_payload: select_source_type.InputPayload): select_source_type.OutputPayload {...}
export function get_torrent(input_payload: get_torrent_type.InputPayload): get_torrent_type.OutputPayload {...}

```
- All additional types and methods available in `import "@plugin_provider/..."`
- You can create ts files, functions, install and import packages as much as you want.
- But there some unsupported packages: 
    - Network (Some methods available in `@plugin_provider`)
    - File IO (Some methods available in `@plugin_provider`)
    - And any other packages that can't be load in `boa_engine`

Test Plugin;
```bash

```


Build Plugin:
```bash
bun run build
```
- Output to `dist/plugin.js`



