use wasmtime::{component::{bindgen, Component, Linker}, Engine, Store};
use wasm_study::greet::greetable::Host;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use crate::cli::Cli;

bindgen!({
    path: "../lib/say/wit",
    world: "sayable-provider"
});

pub struct Greet {
    pub name: String,
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
}

impl Greet {
    pub fn new(name: String) -> Greet {
        let wasi_ctx = WasiCtxBuilder::new().build();
        let resource_table = ResourceTable::new();

        Greet { 
            name, 
            wasi_ctx, 
            resource_table,
        }
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

impl WasiView for Greet {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }
}

pub fn execute(cli: &Cli) -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, Greet::new("Wasm Component".to_string()));
    let component = Component::from_file(&engine, &cli.wasm_file)?;

    SayableProvider::add_to_linker(&mut linker, |greet: &mut Greet| greet)?;
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    let provider = SayableProvider::instantiate(&mut store, &component, &linker)?;

    let message = provider.wasm_study_greet_sayable().call_say(&mut store)?;
    println!("{message}");

    Ok(())
}