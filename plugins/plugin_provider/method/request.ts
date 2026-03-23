import "fast-text-encoding";

export type InputPayload = {
    url: string;
    headers?: Record<string, string>;
    method: "post" | "get" | "put" | "delete" | "patch" | "options" | "head";
    payload?: | Record<string, unknown>
        | unknown[] 
        | string  
        | number
        | boolean;
}

export type OutputPayload = {
    url: string;
    headers?: Record<string, string>;
    status: number;
    body: Uint8Array;
}

export default class request {

    private input_payload: InputPayload;
    public output_payload?: OutputPayload;


    constructor(input_payload: InputPayload) {
        this.input_payload = input_payload;
    }

    async send(): Promise<this> {
        const mode = process.env.NODE_ENV;
        if (mode == "production") {
            this.output_payload = await bridge_request(this.input_payload);
        } else {
            let res = await fetch(this.input_payload.url, {
                method: this.input_payload.method,
                headers: this.input_payload.headers,
                body: this.input_payload.payload as any
            })

            this.output_payload = {
                url: this.input_payload.url,
                headers: this.input_payload.headers,
                status: res.status,
                body: new Uint8Array(await res.arrayBuffer())
            }
        }
        return this;
    }

    body_json(): any {
        if (!this.output_payload) {
            throw new Error("output_payload is not defined yet. Call 'send()' first.");
        }
        return JSON.parse(new TextDecoder().decode(this.output_payload.body));
    }
}


