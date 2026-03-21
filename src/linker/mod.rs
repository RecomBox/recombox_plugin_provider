use boa_engine::{
    Context, JsResult, JsValue, JsString,
    object::{FunctionObjectBuilder},
    native_function::NativeFunction,
    property::Attribute,
    Source,
};

use std::path::Path;

mod request;


pub async fn get_context() {
    
    let mut context = Context::default();

    let func = FunctionObjectBuilder::new(
        context.realm(),
        NativeFunction::from_async_fn(request::new),
    )
    .build();

    // Register the Rust function globally
    context.register_global_property(
        JsString::from("hello"),
        func,
        Attribute::all(),
    ).unwrap();

    let script_path = Path::new(r"D:\Codes\recombox_plugin_provider\plugins\plugin_the_pirate_bay\dist\plugin.js");

    context.eval(Source::from_filepath(script_path).unwrap()).unwrap();

    // Call it from JS
    let result = context.eval(Source::from_bytes("plugin.get_torrent(hello());")).unwrap();
    println!("{:?}", result.to_string(&mut context).unwrap());
}