
pub mod get_plugin_list;
pub mod install_plugin;


use std::path::{PathBuf};
use std::fs::{create_dir_all};
use std::collections::HashMap;
use redb::{Database, Error, ReadableDatabase, TableDefinition, ReadableTable};
use serde::{Deserialize, Serialize};
use postcard;
use blake3;


use crate::global_types::Source;

#[derive(Debug, Deserialize, Serialize)]
pub struct InstalledPluginInfo{
    pub plugin_name: String,
    pub plugin_repo_url: String,
    pub plugin_path: String,
    pub plugin_version: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InstalledPlugins(HashMap<String, InstalledPluginInfo>);



pub struct PluginDatabaseManager{
    pub plugin_directory: PathBuf
}

impl PluginDatabaseManager{
    
    async fn get_db(&self) -> anyhow::Result<Database>{
        

        if !self.plugin_directory.exists() {
            create_dir_all(&self.plugin_directory)?;
        }
        let db_path = self.plugin_directory.join("plugin_db.redb");
        
        let db = Database::create(db_path)?;
        
        return Ok(db);
    }
    
    pub async fn add_plugin(
        &self,
        plugin_source: Source,
        plugin_id: &str,
        installed_plugin_info: InstalledPluginInfo
    ) -> anyhow::Result<()>{
        
        
        let serialize_plugin_info =  postcard::to_allocvec(&installed_plugin_info)?;

        let table: TableDefinition<&str, Vec<u8>> = TableDefinition::new(plugin_source.as_str());
        
        let db = self.get_db().await?;

        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(table)?;
            table.insert(plugin_id, serialize_plugin_info)?;
        }
        
        write_txn.commit()?;
        return Ok(());
    }

    pub async fn get_installed_plugins(
        &self,
        plugin_source: Source
    ) -> anyhow::Result<InstalledPlugins> {
        let db = self.get_db().await?;
        let read_txn = db.begin_read()?;
        let table: TableDefinition<&str, Vec<u8>> = TableDefinition::new(plugin_source.as_str());

        let read_table = match read_txn.open_table(table){
            Ok(table) => table,
            Err(_) => return Ok(InstalledPlugins(HashMap::new())),
        };

        let mut new_installed_plugin: InstalledPlugins = InstalledPlugins(HashMap::new());
        // Iterate over all key-value pairs
        for entry in read_table.iter()? {
            let (k, v) = entry?;
            let key = k.value();
            let value = postcard::from_bytes::<InstalledPluginInfo>(&v.value())?;

            new_installed_plugin.0.insert(key.to_string(), value);
        }

        return Ok(new_installed_plugin);


        return Err(anyhow::Error::msg("Not implemented"));
    }

}