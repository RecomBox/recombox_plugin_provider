#[cfg(test)]
mod tests {
    // ==================================================================
    // Note: Uncomment #[tokio::test(flavor = "multi_thread")] to enable specific test.
    // ==================================================================


    use super::*;
    
    use std::path::Path;
    

    #[tokio::test(flavor = "multi_thread")]
    async fn get_torrent() {
        use crate::get_torrents;

        // -> Stuff that you need to changes
        let input_payload = get_torrents::InputPayload {
            id: "72673844".to_string(),
            source: "anime".to_string(),
        };

        let script_path = Path::new(r"D:\Codes\recombox_plugin_provider\plugins\plugin_the_pirate_bay\dist\plugin.js");
        // <-

        
        let result = get_torrents::new(script_path, input_payload).await;

        println!("{:?}", result);

        
    }


    // #[tokio::test(flavor = "multi_thread")]
    async fn get_source() {
        use crate::get_sources;

        // -> Stuff that you need to changes
        let input_payload = get_sources::InputPayload {
            id: "tt9140554".to_string(),
            title: "Spider Man".to_string(),
            source: "tv".to_string(),
            season: Some(1),
            episode: Some(1),
            search: None
        };

        let script_path = Path::new(r"D:\Codes\recombox_plugin_provider\plugins\plugin_the_pirate_bay\dist\plugin.js");
        // <-

        let result = get_sources::new(script_path, input_payload).await;

        println!("{:?}", result);

        
    }
}
