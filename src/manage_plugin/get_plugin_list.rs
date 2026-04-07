use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Client;
use crate::global_types::Source;

#[derive(Debug, Deserialize, Serialize)]
pub struct InputPayload{
    pub manifest_repo_url: String,
    pub source: Source,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OuputPayloadInfo{
    pub name: String,
    pub repo_url: String,
    pub icon_url: String
}

#[derive(Debug, Deserialize, Serialize)]
// <PLUGIN_ID, PLUGIN_INFO>
pub struct OuputPayload(pub HashMap<String, OuputPayloadInfo>);


pub async fn new(input_payload: InputPayload) -> anyhow::Result<OuputPayload> {

    let url = format!("{}/{}.json", input_payload.manifest_repo_url, input_payload.source.to_string());
    println!("{}", url);

    let client = Client::new();


    let data = client.get(url)
        .send().await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?
        .json::<OuputPayload>().await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;


    return Ok(data);

    // return Err(anyhow::Error::msg("Not implemented"));

    
}