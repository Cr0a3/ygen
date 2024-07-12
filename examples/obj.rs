use std::{error::Error, fs::OpenOptions};

use Ygen::{Obj::*, Target::Triple};

fn main() -> Result<(), Box<dyn Error>> {
    let mut obj = ObjectBuilder::new(
        Triple::parse("x86_64-pc-windows")?
    );

    obj.decls(vec![
        ("test", Decl::Function, Linkage::External),
        ("test_data", Decl::Constant, Linkage::Extern),
    ]);

    obj.define("test", vec![
        0xB8, 0x00, 0x00, 0x00, 0x00, // mov eax, 5   
        0xC3,
    ]);

    obj.link(Link { from: "test".into(), to: "test_data".into(), at: 1 });

    obj.emit(
        OpenOptions::new().create(true).write(true).open("output.o")?
    )?;

    Ok(())
}