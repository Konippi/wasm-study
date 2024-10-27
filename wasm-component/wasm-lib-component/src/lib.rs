#[allow(warnings)]
mod bindings;

use crate::bindings::exports::wasm_study::wasm_lib_component::greetable::Guest;

struct Component;

impl Guest for Component {
    fn name() -> String {
        "Wasm Component".to_string()
    }
    fn greet(name: String) -> String {
        format!("Hello {name}!")
    }
}

bindings::export!(Component with_types_in bindings);
