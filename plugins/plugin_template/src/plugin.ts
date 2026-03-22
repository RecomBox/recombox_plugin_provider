import * as get_sources_types from "@plugin_provider/global_types/get_sources";
import * as get_torrents_types from "@plugin_provider/global_types/get_torrents";

import * as request_types from "@plugin_provider/global_types/request";

export async function get_sources(input_payload: get_sources_types.InputPayload): Promise<get_sources_types.OutputPayload> {
    let data: get_sources_types.OutputPayload = [{
        id: "test_output_id",
        title: "test_output_title",
    }];
    return data;
}

export async function get_torrents(input_payload: get_torrents_types.InputPayload): Promise<get_torrents_types.OutputPayload> {
    let input_request: request_types.InputPayload = {
        url: "https://httpbin.org/get",
        method: "get",
        
    };

    let res = await request(input_request);
    
    let data: get_torrents_types.OutputPayload = [{
        title: res.status.toString(),
        torrent_url: "magnet or torrent_file url",
    }];
    
    return data;
}