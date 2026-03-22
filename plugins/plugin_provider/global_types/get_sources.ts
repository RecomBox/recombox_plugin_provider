
export type InputPayload = {
    id: string;
    title: string;
    source: string;
    season: string;
    episode: string;
    search?: string;
}

export type OutputPayloadInfo = {
    id: string;
    title: string;
}


export type OutputPayload = OutputPayloadInfo[];