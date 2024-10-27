use wasmtime::{component::{bindgen, Component, Linker}, Engine, Store};

use crate::cli::Cli;

bindgen!({
    world: "greetable-provider",
    path: "../wasm-lib-component/wit",
});

pub fn load(cli: &Cli) -> anyhow::Result<()> {
    let engine = Engine::default();
    let component = Component::from_file(&engine, &cli.wasm_file)?;
    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    let provider = GreetableProvider::instantiate(&mut store, &component, &linker)?;
    let greetable = provider.wasm_study_wasm_lib_component_greetable();

    let name = greetable.call_name(&mut store)?;
    println!("name: {name}");

    let message = greetable.call_greet(&mut store, &name)?;
    println!("message: {message}");

    Ok(())
}