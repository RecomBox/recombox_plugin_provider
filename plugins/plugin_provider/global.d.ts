import * as request_types from "./global_types/request";

export {};

declare global{
    function request(input_payload: request_types.InputPayload): Promise<request_types.OutputPayload>;
}