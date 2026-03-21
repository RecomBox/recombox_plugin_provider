use boa_engine::{
    Context, JsResult, JsValue, JsString,
    object::{FunctionObjectBuilder},
    native_function::NativeFunction,
    property::Attribute,
    Source,
    builtins::promise::Promise,
};

use std::cell::RefCell;

pub async fn new(
    this: &JsValue,
    args: &[JsValue],
    ctx: &RefCell<&mut Context>,
) -> JsResult<JsValue> {
    // Borrow the context when you need it
    let mut ctx = ctx.borrow_mut();

    
    Ok(JsValue::new(JsString::from("Hello from Rust async!")))
}