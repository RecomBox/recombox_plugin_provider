
export type InputPayload = {
    id: string;
    source: string;
    page: number;
}

export type OutputPayloadInfo = {
    title: string;
    torrent_url: string;
}


export type OutputPayload = OutputPayloadInfo[];