use serde::{Deserialize, Serialize};
use serde_json::{to_string, from_value};
use std::collections::HashMap;

use crate::global_types::Source;

#[derive(Debug, Deserialize, Serialize)]
pub struct InputPayload{
    pub repo: String,
    pub source: Source,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OuputPayloadInfo{
    pub name: String,
    pub repo: String
}

#[derive(Debug, Deserialize, Serialize)]
// <PLUGIN_ID, PLUGIN_INFO>
pub struct OuputPayload(pub HashMap<String, OuputPayloadInfo>);


pub async fn new(input_payload: InputPayload) -> anyhow::Result<OuputPayload> {

    let url = format!("{}/{}.json", input_payload.repo, input_payload.source.to_string());

    let data = reqwest::get(url)
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?
        .json::<OuputPayload>()
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;


    return Ok(data);
    
}