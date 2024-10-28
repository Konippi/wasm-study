use wasmtime::{component::{bindgen, Component, Linker}, Engine, Store};

use crate::cli::Cli;

bindgen!({
    world: "sayable-provider",
    path: "wit",
});

pub fn load(cli: &Cli) -> anyhow::Result<()> {
    let engine = Engine::default();
    let component = Component::from_file(&engine, &cli.wasm_file)?;
    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    let provider = SayableProvider::instantiate(&mut store, &component, &linker)?;

    let said = provider.wasm_study_say_sayable().call_say(&mut store)?;
    println!("{said}");

    Ok(())
}