import * as select_source_types from "@plugin_provider/global_types/select_source";
import * as get_torrent_types from "@plugin_provider/global_types/get_torrent";

export function select_source(input_payload: select_source_types.InputPayload): select_source_types.OutputPayload {
    
    let data: select_source_types.OutputPayload = [{
        id: "test_output_id",
        title: "test_output_title",
    }];
    
    return data;
}

export function get_torrent(input_payload: get_torrent_types.InputPayload): get_torrent_types.OutputPayload {
    
    let data: get_torrent_types.OutputPayload = [{
        title: "test_output_title",
        torrent_url: "magnet or torrent_file url",
    }];
    
    return data;
}