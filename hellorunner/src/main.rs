use std::collections::HashMap;
use wascc_host::{WasccHost, Actor, NativeCapability};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();

    let host = WasccHost::new();
    host.add_actor(Actor::from_file("../hellohttp/hello_signed.wasm")?)?;
    host.add_actor(Actor::from_file("../hellohttp2/hello_signed.wasm")?)?;
    host.add_native_capability(NativeCapability::from_file(
        "../libwascc_httpsrv.dylib", None)?)?;
    host.add_native_capability(NativeCapability::from_file(
        "../libkeyvalue.dylib", None)?)?;
    host.bind_actor(
        "MDAELGEJDJPPMBHCVTXRCS6TYMEVOSEYLYHPDVWVBBRHNQ6KNIEBHNHQ",
        "wascc:http_server",
        None,
        generate_port_config(8081),
    )?;

    host.bind_actor(
        "MDAELGEJDJPPMBHCVTXRCS6TYMEVOSEYLYHPDVWVBBRHNQ6KNIEBHNHQ",
        "wascc:messaging",
        None,
        HashMap::new(),
    )?;

    host.bind_actor(
        "MAVB7LZ22BBLNW5SL3EZP7A6NTFGZFHFJVSFSPUINTLR5YQXR3LZVO7H",
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
