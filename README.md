# recombox_plugin_provider


## Installation
Make sure you have the following installed:
- [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [NodeJS](https://nodejs.org/)
- [Bun](https://bun.com/) or [NPM](https://www.npmjs.com/) or whatever runtime you want to use


## Making Plugins
Clone this repository
```bash
git clone https://github.com/RecomBox/recombox_plugin_provider
```

Create and Copy:
- Create new folder name `plugins` at the root of the project
- Copy `plugin_template`

Install dependencies:
```bash
bun install
```

Build Plugin:
```bash
bun run build
```

Required export functions inside plugin.ts:
```typescript
export function get_torrent(input_payload: select_source_type.InputPayload): select_source_type.OutputPayload {}


```

