use boa_engine::{
    Context, JsString,
    object::{FunctionObjectBuilder},
    native_function::NativeFunction,
    property::Attribute,
};

mod bridge_request;


pub async fn get_context() -> anyhow::Result<Context>{
    
    let mut context = Context::default();


    // Register the Rust function globally
    context.register_global_property(
        JsString::from("bridge_request"),

        FunctionObjectBuilder::new(
            context.realm(),
            NativeFunction::from_async_fn(bridge_request::new),
        ).build(),

        Attribute::all(),
    ).map_err(|e| anyhow::Error::msg(e.to_string()))?;

    return Ok(context);


    
}