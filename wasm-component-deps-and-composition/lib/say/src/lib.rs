#[allow(warnings)]
mod bindings;

use crate::bindings::exports::wasm_study::greet::sayable::Guest;
use crate::bindings::wasm_study::greet::greetable::{greet, name};

struct Component;

impl Guest for Component {
    fn say() -> String {
        let name = name();
        let message = greet(&name);
        let mut buffer = Vec::new();

        ferris_says::say(&message, 80, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}

bindings::export!(Component with_types_in bindings);
