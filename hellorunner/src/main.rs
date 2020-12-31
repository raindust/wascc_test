use std::collections::HashMap;
use wascc_host::{WasccHost, Actor, NativeCapability};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();

    let host = WasccHost::new();
    host.add_actor(Actor::from_file("../hellohttp/target/wasm32-unknown-unknown/debug/hellohttp_signed.wasm")?)?;
    host.add_actor(Actor::from_file("../hellohttp2/target/wasm32-unknown-unknown/debug/hellohttp_signed.wasm")?)?;
    host.add_native_capability(NativeCapability::from_file(
        "../libwascc_httpsrv.dylib", None)?)?;
    host.add_native_capability(NativeCapability::from_file(
        "../libkeyvalue.dylib", None)?)?;
    host.bind_actor(
        "MDCLGKGFS7OWD23C3LAL3QLSBIBCRQM52TW2D45UKZMZ2AVVLGTB3QMF",
        "wascc:http_server",
        None,
        generate_port_config(8081),
    )?;
    host.bind_actor(
        "MDCLGKGFS7OWD23C3LAL3QLSBIBCRQM52TW2D45UKZMZ2AVVLGTB3QMF",
        "wascc:messaging",
        None,
        HashMap::new(),
    )?;

    host.bind_actor(
        "MBIBIPLIP33XA6EPELRIAYCXJAX2P6RTM6PGC43FWUINTQSHTLISDCKV",
        "wascc:messaging",
        None,
        HashMap::new(),
    )?;

    std::thread::park();

    Ok(())
}

fn generate_port_config(port: u16) -> HashMap<String, String> {
    let mut hm = HashMap::new();
    hm.insert("PORT".to_string(), port.to_string());

    hm
}
