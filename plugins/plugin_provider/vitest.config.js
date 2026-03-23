import { defineConfig } from "vitest/config";

export default defineConfig({
        test: {
        globals: true,
        environment: "node",
        include: ["test.ts"], // tell Vitest to run test.ts
        watch: false
    },
});
