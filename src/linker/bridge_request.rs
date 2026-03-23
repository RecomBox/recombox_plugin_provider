use boa_engine::{
    Context, JsResult, JsValue, JsString,
    object::{FunctionObjectBuilder},
    native_function::NativeFunction,
    property::Attribute,
    Source,
    builtins::promise::Promise,
    JsNativeError,
};

use std::{cell::RefCell, os::raw, str::{Bytes, FromStr}};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value, to_value};
use reqwest::{
    Client, Method,
    header::{HeaderMap, HeaderName, HeaderValue},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct InputPayload{
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub method: String,
    pub payload: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutputPayload{
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub status: u16,
    pub body: Vec<u8>,
}




pub async fn new(
    this: &JsValue,
    args: &[JsValue],
    ref_ctx: &RefCell<&mut Context>,
) -> JsResult<JsValue> {
    
    let args_value = {
        let mut ctx = ref_ctx.borrow_mut();

        args.get(0)
            .ok_or("Unable to parse args")
            .map_err(|e| JsNativeError::typ().with_message(e.to_string()))?
            .to_json(&mut ctx)?
            .ok_or("Unable to parse args")
            .map_err(|e| JsNativeError::typ().with_message(e.to_string()))?
    };


    
    let input_payload: InputPayload = from_value(args_value)
        .map_err(|e| JsNativeError::typ().with_message(e.to_string()))?;

    
    let method = match input_payload.method.as_str() {
        "get" => Method::GET,
        "post" => Method::POST,
        "put" => Method::PUT,
        "delete" => Method::DELETE,
        "patch" => Method::PATCH,
        "options" => Method::OPTIONS,
        "head" => Method::HEAD,
        _ => Method::GET,
    };

    let client = Client::new();
    let mut response = client.request(method, &input_payload.url);

    if let Some(payload)= input_payload.payload {
        response = response.json(&payload);
    }

    if let Some(raw_headers) = &input_payload.headers {
        let mut new_headermap = HeaderMap::new();

        for (key, value) in raw_headers {
            
            new_headermap.insert(
                HeaderName::from_str(key)
                    .map_err(|e| JsNativeError::typ().with_message(e.to_string()))?,
                    
                HeaderValue::from_str(value)
                    .map_err(|e| JsNativeError::typ().with_message(e.to_string()))?,
            );
        }

        response = response.headers(new_headermap);
    }

    
    let response = response
        .send().await
        .map_err(|e| JsNativeError::typ().with_message(e.to_string()))?;

    

    let url = response.url().to_string();
    let status = response.status().as_u16();


    let body = response
        .bytes()
        .await
        .map_err(|e| JsNativeError::typ().with_message(e.to_string()))?
        .to_vec();
    
    let output_payload = OutputPayload {
        url,
        headers: input_payload.headers,
        status,
        body,
    };

    let output_payload_to_value = to_value(output_payload)
        .map_err(|e| JsNativeError::typ().with_message(e.to_string()))?;

    

    
    let js_value = {
        let mut ctx = ref_ctx.borrow_mut();

        JsValue::from_json(&output_payload_to_value,  &mut ctx)?
    };

    return Ok(js_value);

}