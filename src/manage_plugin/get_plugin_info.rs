use super::{PluginInfo};


pub async fn new(plugin_repo_url: &str) -> anyhow::Result<PluginInfo> {

    let url = format!("{}/releases/latest/download/latest.json", plugin_repo_url);

    let data = reqwest::get(url)
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?
        .json::<PluginInfo>()
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    return Ok(data);
    
}