use boa_engine::{
    Context, JsResult, JsValue, JsString,
    object::{FunctionObjectBuilder},
    native_function::NativeFunction,
    property::Attribute,
    Source,
};

use std::path::Path;

fn hello_world(
    this: &JsValue,
    args: &[JsValue],
    ctx: &mut Context,
) -> JsResult<JsValue> {
    Ok(JsValue::new(JsString::from("Hello from Rust!")))
}


pub fn get_context() {
    
    let mut context = Context::default();

    let func = FunctionObjectBuilder::new(
        context.realm(),
        NativeFunction::from_fn_ptr(hello_world),
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