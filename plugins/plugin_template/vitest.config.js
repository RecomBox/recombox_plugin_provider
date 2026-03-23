import { defineConfig } from "vitest/config";

export default defineConfig({
    resolve: {
        tsconfigPaths: true, // <-- tells Vite/Vitest to use tsconfig.json paths
    },
    test: {
        globals: true,
        environment: "node",
        include: ["test.ts"], // run only test.ts
        watch: false,
    },
});
