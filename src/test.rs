#[cfg(test)]
mod tests {
    // ==================================================================
    // Note: Uncomment #[tokio::test(flavor = "multi_thread")] to enable specific test.
    // ==================================================================

    
    use std::path::{PathBuf, Path};
    

    #[tokio::test(flavor = "multi_thread")]
    async fn get_plugin_list() {
        use crate::manage_plugin::get_plugin_list;
        use crate::global_types::Source;

        let input_payload = get_plugin_list::InputPayload {
            manifest_repo: "https://raw.githubusercontent.com/RecomBox/recombox_plugin_provider/refs/heads/main/plugins_manifest".to_string(),
            source: Source::Anime
        };

        let result = get_plugin_list::new(input_payload).await.unwrap();
        println!("{:?}", result);
    }


    // #[tokio::test(flavor = "multi_thread")]
    async fn get_installed_plugins() {
        use crate::manage_plugin::PluginDatabaseManager;
        use crate::global_types::Source;

        let plugin_db_manager = PluginDatabaseManager{
            plugin_directory: PathBuf::from("./"),
        };

        let result = plugin_db_manager.get_installed_plugins(Source::Movies).await.unwrap();
        println!("{:?}", result);
    }

    // #[tokio::test(flavor = "multi_thread")]
    async fn get_torrent() {
        use crate::get_torrents;
        use crate::global_types::Source;

        // -> Stuff that you need to changes
        let input_payload = get_torrents::InputPayload {
            id: "72673844".to_string(),
            source: Source::Anime,
            page: 1
        };

        let script_path = Path::new(r"D:\Codes\recombox_plugin_provider\plugins\plugin_the_pirate_bay\dist\plugin.js");
        // <-


        let result = get_torrents::new(script_path, input_payload).await;

        println!("{:?}", result);

        
    }


    // #[tokio::test(flavor = "multi_thread")]
    async fn get_source() {
        use crate::get_sources;
        use crate::global_types::Source;

        // -> Stuff that you need to changes
        let input_payload = get_sources::InputPayload {
            id: "tt9140554".to_string(),
            title: "Spider Man".to_string(),
            source: Source::Tv,
            season: Some(1),
            episode: Some(1),
            search: None,
            page: 1
        };

        let script_path = Path::new(r"D:\Codes\recombox_plugin_provider\plugins\plugin_the_pirate_bay\dist\plugin.js");
        // <-

        let result = get_sources::new(script_path, input_payload).await;

        println!("{:?}", result);

        
    }
}
