import * as request_types from "request";

declare global{
    function bridge_request(input_payload: request_types.InputPayload): Promise<request_types.OutputPayload>;

}




