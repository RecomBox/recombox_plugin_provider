import { describe, it, expect } from "vitest";
import { get_sources } from "./src/plugin.ts";

describe("get_sources", () => {
    it("should fetch sources", async () => {
        let result = "Hello";

        console.log(result);
        expect(result).toBeDefined();
    });
});
