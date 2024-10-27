use wasmtime::{component::{Component, Linker, TypedFunc}, Engine, Store};
use anyhow;

use crate::cli::Cli;

pub fn load(cli: &Cli) -> anyhow::Result<()> {
    let engine = Engine::default();
    let component = Component::from_file(&engine, &cli.wasm_file)?;
    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    let instance = linker.instantiate(&mut store, &component)?;

    let greetable_index = instance.get_export(&mut store, None, "wasm-study:wasm-lib-component/greetable").unwrap_or_else(|| {
        panic!("Failed to find `wasm-study:wasm-lib-component/greetable` export")
    });
    let greet_index = instance.get_export(&mut store, Some(&greetable_index), "greet").unwrap_or_else(|| {
        panic!("Failed to find `greet` export in `wasm-study:wasm-lib-component/greetable`")
    });
    let name_index = instance.get_export(&mut store, Some(&greetable_index), "name").unwrap_or_else(|| {
        panic!("Failed to find `name` export in `wasm-study:wasm-lib-component/greetable`")
    });
    let greet_fn: TypedFunc<(String, ), (String, )> = instance.get_typed_func(&mut store, greet_index)?;
    let name_fn: TypedFunc<(), (String, )> = instance.get_typed_func(&mut store, name_index)?;

    let (name, ) = name_fn.call(&mut store, ())?;
    name_fn.post_return(&mut store)?;
    println!("name: {name}");

    let (message, ) = greet_fn.call(&mut store, (name, ))?;
    greet_fn.post_return(&mut store)?;
    println!("message: {message}");

    Ok(())
}