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

Install dependencies:
```bash
# Or use any runtime you want
bun install
```

**[Required]** export functions inside `plugin.ts`:
```typescript
import * as get_sources_types from "@plugin_provider/global_types/get_sources";
import * as get_torrents_types from "@plugin_provider/global_types/get_torrents";

export function get_sources(input_payload: get_sources_types.InputPayload): get_sources_types.OutputPayload {...}
export function get_torrents(input_payload: get_torrents_types.InputPayload): get_torrents_types.OutputPayload {...}

```
- All additional types and methods available in `import "@plugin_provider/..."` and global declared functions.
- You can create ts files, functions, install and import packages as much as you want.
- But there some unsupported packages: 
    - Network (Some methods available in global declared functions)
    - File IO (Some methods available in global declared functions)
    - And any other packages that can't be load in `boa_engine`

Build Plugin:
```bash
bun run build
```
- Output to `dist/plugin.js`


Test Plugin;
- **[Required]** rebuild the plugin using build command from above everytime you make changes to the plugin.
- Write and modify test value inside rust `/src/test.rs`
```bash
cargo test -- --nocapture

# Or
cargo test

```






