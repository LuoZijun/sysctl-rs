extern crate sysctl;

use std::io;

fn main() -> Result<(), io::Error> {
    // net.inet.ip.forwarding
    // kern.ostype
    // let root = "kern".parse::<sysctl::Mib>()?;
    let root = sysctl::Mib::default();
    for mib_res in root {
        let mib = mib_res?;
        let name = mib.name()?;

        match mib.metadata() {
            Ok(metadata) => {
                if metadata.is_struct() {
                    match mib.value() {
                        Ok(value) => {
                            if value.size() > 20 {
                                println!("{} = ...", name);
                            } else {
                                println!("{} = {:?}", name, value);
                            }
                        }
                        Err(e) => {
                            println!("{} = {}", name, e);
                        }
                    }
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    // PASS
                } else {
                    println!("{}", name);
                }
            }
        }
    }

    Ok(())
}
