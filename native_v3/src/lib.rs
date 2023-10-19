#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod mappers;
mod messaging;
mod typescript;
mod writer;
mod schemy;

use std::collections::{HashMap, BTreeMap};

use mappers::{
    open_api::{OpenApiMapper, OpenApiResult},
    Mapper,
};
use messaging::MessageBus;
use serde::{Deserialize, Serialize};
use typescript::Application;

#[derive(Deserialize)]
struct GenerateSchemaOptions {
    pub open_api: Option<mappers::open_api::OpenApiOptions>,
}

#[derive(Default, Serialize)]
pub struct GenerateSchemaResult {
    pub open_api: Option<OpenApiResult>,
}

pub fn generate_schemas_debug(open_api_options: mappers::open_api::OpenApiOptions) -> GenerateSchemaResult {
    let mut result = GenerateSchemaResult::default();
    let bus = MessageBus::new();
    let mut application = Application::new();
    let mut modules = BTreeMap::<String, usize>::new();

    let handle = OpenApiMapper::run(Some(open_api_options), bus.clone());

    application.run(bus);

    if let Some(handle) = handle {
        let open_api = handle.join().unwrap();
        result.open_api = Some(open_api);
    }

    return result;
}

// fn generate_schemas(mut cx: FunctionContext) -> Result<Handle<JsString>, Throw> {
//     let mut application = Application::default();
//     let (request_module, on_request_module) = crossbeam::channel::unbounded::<String>();
//     let (send_module, on_send_module) = crossbeam::channel::unbounded();

//     let mut result = GenerateSchemaResult::default();
//     let string_options = cx.argument::<JsString>(0)?.value(&mut cx);
//     let options = serde_json::from_str::<GenerateSchemaOptions>(&string_options).unwrap();

//     on_request_module.iter().for_each(|p| {
//         let module = application.get_module(&p);
//         send_module.send((p, module)).unwrap();
//     });

//     let open_api_handle =
//         mappers::open_api::OpenApiMapper::run(options.open_api, request_module, on_send_module);

//     if let Some(handle) = open_api_handle {
//         result.open_api = Some(handle.join().unwrap());
//     }

//     Ok(cx.string(serde_json::to_string(&result).unwrap()))
// }

// #[neon::main]
// fn main(mut cx: ModuleContext) -> NeonResult<()> {
//     cx.export_function("generateSchemas", generate_schemas)?;
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{generate_schemas_debug, mappers::open_api::OpenApiOptions};

    #[test]
    fn sends_open_api_options_to_open_api_mapper() {
        let timer = std::time::Instant::now();
        let filepaths = env::var_os("API_PATHS");
        let filepaths = serde_json::from_str::<Vec<String>>(filepaths.unwrap().to_str().unwrap()).unwrap();

        generate_schemas_debug(OpenApiOptions {
            output: None,
            base: String::from("{}"),
            filepaths,
        });
        println!("Elapsed: {:?} milliseconds", timer.elapsed().as_millis());
    }
}