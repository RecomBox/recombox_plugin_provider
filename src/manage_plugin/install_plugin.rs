use serde::{Deserialize, Serialize};
use std::fs::{File, create_dir_all};
use std::path::PathBuf;
use std::io::{BufWriter, copy};
use futures_util::stream::StreamExt;

use crate::global_types::Source;

use super::{InstalledPluginInfo, PluginDatabaseManager};

#[derive(Debug, Deserialize, Serialize)]
pub struct InputPayload{
    pub hashed_manifest_repo_id: String,
    pub plugin_directory: PathBuf,
    pub plugin_source: Source,
    pub plugin_id: String,
    pub plugin_repo_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OuputPayload{
    pub name: String,
    pub version: String,
    pub url: String,
    pub icon_url: String
}



pub async fn new(input_payload: InputPayload) -> anyhow::Result<()> {

    let url = format!("{}/releases/latest/download/latest.json", input_payload.plugin_repo_url);


    let data = reqwest::get(url)
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?
        .json::<OuputPayload>()
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    let plugin_file_url = data.url;


    let plugin_path = PathBuf::from(input_payload.plugin_source.as_str())
        .join(&input_payload.hashed_manifest_repo_id)
        .join(&format!("{}.js", input_payload.plugin_id));
    

    let new_installed_plugin = InstalledPluginInfo{
        hashed_manifest_repo_id: input_payload.hashed_manifest_repo_id.to_string(),
        plugin_name: data.name,
        plugin_version: data.version,
        plugin_repo_url: input_payload.plugin_repo_url.to_string(),
        plugin_path: plugin_path.to_string_lossy().to_string(),
        plugin_icon_url: data.icon_url
    };

    let plugin_full_path = input_payload.plugin_directory
        .join(plugin_path);
    
    let response = reqwest::get(plugin_file_url).await?;

    // -> Make sure directory Exists
    let plugin_full_parent_dir = input_payload.plugin_directory
        .join(input_payload.plugin_source.as_str())
        .join(&input_payload.hashed_manifest_repo_id);

    if !plugin_full_parent_dir.exists() {
        create_dir_all(&plugin_full_parent_dir)?;
    }
    // <-

    let plugin_file = File::create(&plugin_full_path)?;

    let mut writer = BufWriter::new(plugin_file);
    let mut content = response.bytes_stream();

    while let Some(chunk) = content.next().await {
        let chunk = chunk?;
        copy(&mut chunk.as_ref(), &mut writer)?;
    }

    let plugin_db_manager = PluginDatabaseManager{
        plugin_directory: input_payload.plugin_directory,
    };

    plugin_db_manager.add_plugin(
        input_payload.plugin_source,
        &input_payload.plugin_id,
        new_installed_plugin
    ).await?;
    

    return Ok(());
    
}