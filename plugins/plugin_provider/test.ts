import { describe, it} from "vitest";
import request from "./method/request";

describe("test", () => {
    it("test", async () => {
        let result = await new request({ method: "get", url: "https://www.google.com"}).send();


        console.log(result.output_payload?.url);
    });
});
