use std::{error::Error, fs::OpenOptions};

use Ygen::{Obj::*, Target::Triple};

fn main() -> Result<(), Box<dyn Error>> {
    let mut obj = ObjectBuilder::new(
        Triple::parse("x86_64-pc-windows")?
    );

    obj.decls(vec![
        ("main", Decl::Function, Linkage::External),
        ("string", Decl::Constant, Linkage::Internal),
        ("puts", Decl::Function, Linkage::Extern),
    ]);

    let mut data = vec![0x55];

    data.extend_from_slice(&[0x48, 0x83, 0xc4, 0x20]);

    if cfg!(target_os = "windows") {
        data.extend_from_slice(&[0x48, 0x8d, 0x0d, 0x00, 0x00, 0x00, 0x00])
    } else {
        data.extend_from_slice(&[0x48, 0x8d, 0x3d, 0x00, 0x00, 0x00, 0x00])
    }
    
    obj.link( Link { from: "main".into(), to: "string".into(), at: data.len() -4, addend: -4 });

    data.extend_from_slice(&[0xE8, 0x00, 0x00, 0x00, 0x00]);

    obj.link( Link { from: "main".into(), to: "puts".into(), at: data.len() -4, addend: -4});

    data.extend_from_slice(&[0x31, 0xC0]);
    data.extend_from_slice(&[0x48, 0x83, 0xEC, 0x20]);
    data.extend_from_slice(&[0xC3]);

    obj.define("main", data);
    obj.define("string", b"Hello World!\00".into());

    obj.emit(
        OpenOptions::new().create(true).write(true).open("output.o")?
    )?;

    Ok(())
}