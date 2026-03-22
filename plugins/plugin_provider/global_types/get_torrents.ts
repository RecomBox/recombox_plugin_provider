
export type InputPayload = {
    id: string;
    source: string;
}

export type OutputPayloadInfo = {
    title: string;
    torrent_url: string;
}


export type OutputPayload = OutputPayloadInfo[];