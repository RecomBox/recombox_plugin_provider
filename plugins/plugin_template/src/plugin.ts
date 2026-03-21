import * as select_source_type from "./global_types/select_source";
import * as get_torrent_type from "./global_types/get_torrent";

export function select_source(input_payload: select_source_type.InputPayload): select_source_type.OutputPayload {
    
    let data: select_source_type.OutputPayload = [{
        id: "test_output_id",
        title: "test_output_title",
    }];
    
    return data;
}

export function get_torrent(input_payload: get_torrent_type.InputPayload): get_torrent_type.OutputPayload {
    
    let data: get_torrent_type.OutputPayload = [{
        title: "test_output_title",
        torrent_url: "magnet or torrent_file url",
    }];
    
    return data;
}