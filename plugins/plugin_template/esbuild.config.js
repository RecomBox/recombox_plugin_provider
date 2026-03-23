import { build } from "esbuild";

import { NodeModulesPolyfillPlugin } from "@esbuild-plugins/node-modules-polyfill";
import { NodeGlobalsPolyfillPlugin } from "@esbuild-plugins/node-globals-polyfill";

await build({
    entryPoints: ["src/plugin.ts"],   // main entry, imports like test.ts are included
    bundle: true,                     // follow imports and bundle them
    platform: "node",              // avoid Node built-ins
    format: "iife",                   // wrap into one global file
    minify: true,                     // shrink internal helpers
    outfile: "dist/plugin.js",        // output location
    globalName: "plugin",             // exposed as Plugin.get_torrent / Plugin.select_source
    keepNames: true,                  // preserve exported function names
    define: {
        "process.env.NODE_ENV": JSON.stringify(process.env.NODE_ENV || "development"),
    },
    plugins: [
        NodeModulesPolyfillPlugin(),
        NodeGlobalsPolyfillPlugin({
            buffer: true,
            process: true,
        }),
    ],
});
