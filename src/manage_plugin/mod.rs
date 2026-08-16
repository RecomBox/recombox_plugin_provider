pub mod get_plugin_list;
pub mod install_plugin;
pub mod get_plugin_info;

use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio_rusqlite::Connection;

use crate::global_types::Source;

const DEFAULT_MANIFEST_REPO_NAME: &str = "RecomBox";
const DEFAULT_MANIFEST_REPO_URL: &str = "https://raw.githubusercontent.com/RecomBox/recombox_plugin_provider/refs/heads/main/plugins_manifest";

const DATABASE_NAME: &str = "plugin_db.sqlite";


#[derive(Debug, Deserialize, Serialize)]
pub struct PluginInfo{
    pub name: String,
    pub version: String,
    pub url: String,
    pub icon_url: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InstalledManifestRepoInfo{
    pub manifest_repo_name: String,
    pub manifest_repo_url: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InstalledManifestRepo(pub HashMap<String, InstalledManifestRepoInfo>);


#[derive(Debug, Deserialize, Serialize)]
pub struct InstalledPluginInfo{
    pub hashed_manifest_repo_id: String,
    pub plugin_name: String,
    pub plugin_repo_url: String,
    pub plugin_icon_url: String,
    pub plugin_path: String,
    pub plugin_version: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InstalledPlugins(pub HashMap<String, InstalledPluginInfo>);


// Cached per plugin_directory, so re-using the same directory doesn't
// reopen the sqlite file on every call, while a different directory
// (e.g. in tests) still gets its own connection.
static DATABASE_CACHE: Lazy<RwLock<HashMap<PathBuf, Arc<Connection>>>> = Lazy::new(|| RwLock::new(HashMap::new()));


pub struct PluginDatabaseManager{
    pub plugin_directory: PathBuf
}

impl PluginDatabaseManager{

    async fn get_db(&self) -> anyhow::Result<Arc<Connection>> {
        // Fast path: the guard is only ever held for a synchronous clone,
        // never across an `.await`, so this future stays `Send`.
        {
            let read_guard = DATABASE_CACHE.read()
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;

            if let Some(db) = read_guard.get(&self.plugin_directory).cloned() {
                return Ok(db);
            }
        }

        // Slow path: open the connection and create the schema *before*
        // taking any lock, since `std::sync::RwLock` guards aren't `Send`
        // and must never be held across an `.await` point.
        if !self.plugin_directory.exists() {
            create_dir_all(&self.plugin_directory)?;
        }

        let db_path = self.plugin_directory.join(DATABASE_NAME);

        let conn = Connection::open(&db_path).await?;

        conn.call(|conn| -> Result<(), tokio_rusqlite::rusqlite::Error> {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS manifest_repos (
                    hashed_manifest_repo_id TEXT PRIMARY KEY,
                    manifest_repo_name      TEXT NOT NULL,
                    manifest_repo_url       TEXT NOT NULL
                )",
                [],
            )?;

            conn.execute(
                "CREATE TABLE IF NOT EXISTS installed_plugins (
                    plugin_source           TEXT NOT NULL,
                    plugin_id               TEXT NOT NULL,
                    hashed_manifest_repo_id TEXT NOT NULL,
                    plugin_name             TEXT NOT NULL,
                    plugin_repo_url         TEXT NOT NULL,
                    plugin_icon_url         TEXT NOT NULL,
                    plugin_path             TEXT NOT NULL,
                    plugin_version          TEXT NOT NULL,
                    PRIMARY KEY (plugin_source, plugin_id)
                )",
                [],
            )?;

            Ok(())
        }).await?;

        let db = Arc::new(conn);

        // Check-and-store under the write lock. This block is fully
        // synchronous (no `.await` inside it), so the guard never
        // crosses an await point.
        {
            let mut write_guard = DATABASE_CACHE.write()
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;

            if let Some(existing) = write_guard.get(&self.plugin_directory).cloned() {
                return Ok(existing);
            }

            write_guard.insert(self.plugin_directory.clone(), db.clone());
        }

        Ok(db)
    }


    pub async fn add_manifest_repo(
        &self,
        hashed_manifest_repo_id: &str,
        installed_manifest_repo_info: InstalledManifestRepoInfo,
    ) -> anyhow::Result<()>{

        let db = self.get_db().await?;

        let hashed_manifest_repo_id = hashed_manifest_repo_id.to_string();

        db.call(move |conn| -> Result<(), tokio_rusqlite::rusqlite::Error> {
            conn.execute(
                "INSERT INTO manifest_repos (hashed_manifest_repo_id, manifest_repo_name, manifest_repo_url)
                 VALUES (?1, ?2, ?3)
                 ON CONFLICT(hashed_manifest_repo_id) DO UPDATE SET
                    manifest_repo_name = excluded.manifest_repo_name,
                    manifest_repo_url  = excluded.manifest_repo_url",
                tokio_rusqlite::rusqlite::params![
                    hashed_manifest_repo_id,
                    installed_manifest_repo_info.manifest_repo_name,
                    installed_manifest_repo_info.manifest_repo_url
                ],
            )?;

            Ok(())
        }).await?;

        Ok(())
    }

    pub async fn get_installed_manifest_repo(
        &self,
    ) -> anyhow::Result<InstalledManifestRepo> {

        let db = self.get_db().await?;

        let mut new_installed_manifest_repo: InstalledManifestRepo = InstalledManifestRepo(HashMap::new());

        // -> Apply Default Repo First
        let default_hashed_manifest_repo_id = blake3::hash(DEFAULT_MANIFEST_REPO_URL.as_bytes()).to_hex().to_string();
        new_installed_manifest_repo.0.insert(
            default_hashed_manifest_repo_id.clone(),
            InstalledManifestRepoInfo{
                manifest_repo_name: DEFAULT_MANIFEST_REPO_NAME.to_string(),
                manifest_repo_url: DEFAULT_MANIFEST_REPO_URL.to_string()
            }
        );
        // <-

        let rows = db.call(|conn| -> Result<Vec<(String, String, String)>, tokio_rusqlite::rusqlite::Error> {
            let mut stmt = conn.prepare(
                "SELECT hashed_manifest_repo_id, manifest_repo_name, manifest_repo_url FROM manifest_repos"
            )?;

            let rows = stmt.query_map([], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })?;

            let mut result = Vec::new();
            for row in rows {
                result.push(row?);
            }

            Ok(result)
        }).await?;

        for (hashed_manifest_repo_id, manifest_repo_name, manifest_repo_url) in rows {
            new_installed_manifest_repo.0.insert(
                hashed_manifest_repo_id,
                InstalledManifestRepoInfo{ manifest_repo_name, manifest_repo_url }
            );
        }

        Ok(new_installed_manifest_repo)
    }


    pub async fn add_plugin(
        &self,
        plugin_source: Source,
        plugin_id: &str,
        installed_plugin_info: InstalledPluginInfo
    ) -> anyhow::Result<()>{

        let db = self.get_db().await?;

        let plugin_source = plugin_source.as_str().to_string();
        let plugin_id = plugin_id.to_string();

        db.call(move |conn| -> Result<(), tokio_rusqlite::rusqlite::Error> {
            conn.execute(
                "INSERT INTO installed_plugins (
                    plugin_source, plugin_id, hashed_manifest_repo_id,
                    plugin_name, plugin_repo_url, plugin_icon_url,
                    plugin_path, plugin_version
                 )
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                 ON CONFLICT(plugin_source, plugin_id) DO UPDATE SET
                    hashed_manifest_repo_id = excluded.hashed_manifest_repo_id,
                    plugin_name             = excluded.plugin_name,
                    plugin_repo_url         = excluded.plugin_repo_url,
                    plugin_icon_url         = excluded.plugin_icon_url,
                    plugin_path             = excluded.plugin_path,
                    plugin_version          = excluded.plugin_version",
                tokio_rusqlite::rusqlite::params![
                    plugin_source,
                    plugin_id,
                    installed_plugin_info.hashed_manifest_repo_id,
                    installed_plugin_info.plugin_name,
                    installed_plugin_info.plugin_repo_url,
                    installed_plugin_info.plugin_icon_url,
                    installed_plugin_info.plugin_path,
                    installed_plugin_info.plugin_version,
                ],
            )?;

            Ok(())
        }).await?;

        Ok(())
    }


    pub async fn remove_plugin(
        &self,
        hashed_manifest_repo_id: &str,
        plugin_source: Source,
        plugin_id: &str
    ) -> anyhow::Result<()>{

        let db = self.get_db().await?;

        let plugin_source_col = plugin_source.as_str().to_string();
        let plugin_id_col = plugin_id.to_string();

        db.call(move |conn| -> Result<(), tokio_rusqlite::rusqlite::Error> {
            conn.execute(
                "DELETE FROM installed_plugins WHERE plugin_source = ?1 AND plugin_id = ?2",
                tokio_rusqlite::rusqlite::params![plugin_source_col, plugin_id_col],
            )?;

            Ok(())
        }).await?;

        let plugin_full_path = self.plugin_directory
            .join(plugin_source.as_str())
            .join(hashed_manifest_repo_id)
            .join(format!("{}.js", plugin_id));

        if plugin_full_path.exists() {
            std::fs::remove_file(plugin_full_path)?;
        }

        Ok(())
    }

    pub async fn get_installed_plugins(
        &self,
        plugin_source: Source
    ) -> anyhow::Result<InstalledPlugins> {

        let db = self.get_db().await?;

        let plugin_source = plugin_source.as_str().to_string();

        let rows = db.call(move |conn| -> Result<Vec<(String, InstalledPluginInfo)>, tokio_rusqlite::rusqlite::Error> {
            let mut stmt = conn.prepare(
                "SELECT plugin_id, hashed_manifest_repo_id, plugin_name, plugin_repo_url,
                        plugin_icon_url, plugin_path, plugin_version
                 FROM installed_plugins
                 WHERE plugin_source = ?1"
            )?;

            let rows = stmt.query_map(
                tokio_rusqlite::rusqlite::params![plugin_source],
                |row| {
                    let plugin_id: String = row.get(0)?;

                    Ok((plugin_id, InstalledPluginInfo{
                        hashed_manifest_repo_id: row.get(1)?,
                        plugin_name: row.get(2)?,
                        plugin_repo_url: row.get(3)?,
                        plugin_icon_url: row.get(4)?,
                        plugin_path: row.get(5)?,
                        plugin_version: row.get(6)?,
                    }))
                }
            )?;

            let mut result = Vec::new();
            for row in rows {
                result.push(row?);
            }

            Ok(result)
        }).await?;

        let mut new_installed_plugin = InstalledPlugins(HashMap::new());

        for (plugin_id, info) in rows {
            new_installed_plugin.0.insert(plugin_id, info);
        }

        Ok(new_installed_plugin)
    }

}
