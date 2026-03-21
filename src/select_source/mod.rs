use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InputPayload{
    pub id: String,
    pub title: String,
    pub source: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OuputPayloadInfo{
    pub id: String,
    pub title: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OuputPayload(Vec<OuputPayloadInfo>);