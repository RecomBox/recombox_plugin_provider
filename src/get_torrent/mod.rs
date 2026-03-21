use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InputPayload{
    pub id: String,
    pub source: String
}


#[derive(Debug, Deserialize, Serialize)]
pub struct OuputPayloadInfo{
    pub title: String,
    pub torrent_url: String
}


#[derive(Debug, Deserialize, Serialize)]
pub struct OuputPayload( pub Vec<OuputPayloadInfo>);
