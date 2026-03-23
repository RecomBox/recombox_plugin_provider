import type * as get_sources_types from "@plugin_provider/global/types/get_sources";
import type * as get_torrents_types from "@plugin_provider/global/types/get_torrents";


export async function get_sources(input_payload: get_sources_types.InputPayload): Promise<get_sources_types.OutputPayload> {
    throw new Error("Not implemented");
}

export async function get_torrents(input_payload: get_torrents_types.InputPayload): Promise<get_torrents_types.OutputPayload> {
    throw new Error("Not implemented");
}