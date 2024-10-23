use std::env;
use wasmtime::*;
fn main() -> Result<()> {
    let engine = Engine::default();
    let mut args = env::args();
    let fname = args.nth(1).unwrap_or("test.wasm".into());

    println!("Compiling...");
    let m = Module::from_file(&engine, fname)?;

    println!("Initializing");
    let mut s = Store::new(&engine, ());

    let instance = Instance::new(&mut s, &m, &[])?;

    // Next we poke around a bit to extract the `run` function from the module.
    let add = instance.get_typed_func::<(u64, u64), u64>(&mut s, "add")?;
    let (a, b) = (222u64, 2344u64);
    println!("add({}, {}) = {}", a, b, add.call(&mut s, (a, b))?);

    println!("Done.");
    Ok(())
}
