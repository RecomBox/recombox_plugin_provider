
use boa_engine::{
    Context, JsResult, JsValue, JsString,
    object::{FunctionObjectBuilder},
    native_function::NativeFunction,
    property::Attribute,
    Source, Script
};
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::{to_string, from_value};

use crate::linker;

#[derive(Debug, Deserialize, Serialize)]
pub struct InputPayload{
    pub id: String,
    pub source: String,
    pub page: u64
}


#[derive(Debug, Deserialize, Serialize)]
pub struct OuputPayloadInfo{
    pub title: String,
    pub torrent_url: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OuputPayload(Vec<OuputPayloadInfo>);

pub async fn new(script_path: &Path, input_payload: InputPayload) -> anyhow::Result<OuputPayload> {

    let mut context = linker::get_context().await?;

    let source_script = Source::from_filepath(script_path)?;
        
    context.eval(source_script)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
    

    let script = Script::parse(
        Source::from_bytes(&format!(
            r#"plugin.get_torrents({});"#, 
            to_string(&input_payload)?
        )), 
        None, &mut context
    ).map_err(|e| anyhow::Error::msg(e.to_string()))?;

    let promise = script.evaluate_async(&mut context).await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
    
    context.run_jobs().map_err(|e| anyhow::Error::msg(e.to_string()))?;
    
    let result = promise.as_promise()
        .ok_or("Unable to get promise from executed function")
        .map_err(|e| anyhow::Error::msg(e.to_string()))?
        .await_blocking(&mut context)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;


    let reuslt = result.to_json(&mut context)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?
        .ok_or("Unable to get value from executed function")
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;


    let output_payload: OuputPayload = from_value(reuslt)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    return Ok(output_payload);

}