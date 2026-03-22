
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

