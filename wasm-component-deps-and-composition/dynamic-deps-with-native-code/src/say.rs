use wasmtime::{component::{bindgen, Component, Linker}, Engine, Store};
use wasm_study::greet::greetable::Host;

use crate::cli::Cli;

bindgen!({
    path: "../lib/say/wit",
    world: "sayable-provider"
});

pub struct Greet {
    pub name: String,
}

impl Greet {
    pub fn new(name: String) -> Greet {
        Greet { name }
    }
}

impl Host for Greet {
    fn name(&mut self) -> String {
        self.name.clone()
    }
    fn greet(&mut self, name: String) -> String {
        format!("Hello, {name}!")
    }
}

pub fn execute(cli: &Cli) -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, Greet::new("Wasm Component".to_string()));
    let component = Component::from_file(&engine, &cli.wasm_file)?;

    SayableProvider::add_to_linker(&mut linker, |greet: &mut Greet| greet)?;
    let provider = SayableProvider::instantiate(&mut store, &component, &linker)?;

    let message = provider.wasm_study_greet_sayable().call_say(&mut store)?;
    println!("{message}");

    Ok(())
}