extern crate wapc;
use std::fs::File;
use std::io::prelude::*;
use wapc::prelude::*;

fn load_file() -> Vec<u8> {
    let mut f = File::open(".assets/hello.wasm").unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    buf
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let module_bytes = load_file();
    let mut host = WapcHost::new_with_logger(host_callback, &module_bytes, logger, None)?;

    println!("Calling guest (wasm) function");
    let res = host.call("wapc:sample!Hello", b"this is a test")?;
    println!("Result - {}", ::std::str::from_utf8(&res).unwrap());

    Ok(())
}

fn host_callback(
    id: u64,
    ns: &str,
    op: &str,
    payload: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    println!(
        "Guest {} invoked '{}:{}' with payload of {}",
        id,
        ns,
        op,
        ::std::str::from_utf8(payload).unwrap()
    );
    Ok(vec![])
}

fn logger(id: u64, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("***> [{}] {}", id, msg);
    Ok(())
}
