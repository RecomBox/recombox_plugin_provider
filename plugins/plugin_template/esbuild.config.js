import { build } from "esbuild";

await build({
    entryPoints: ["src/plugin.ts"],   // main entry, imports like test.ts are included
    bundle: true,                     // follow imports and bundle them
    platform: "browser",              // avoid Node built-ins
    format: "iife",                   // wrap into one global file
    minify: true,                     // shrink internal helpers
    outfile: "dist/plugin.js",        // output location
    globalName: "plugin",             // exposed as Plugin.get_torrent / Plugin.select_source
    keepNames: true,                  // preserve exported function names
});
