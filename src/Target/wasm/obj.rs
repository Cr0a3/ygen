use std::error::Error;

use crate::{debug::DebugRegistry, Obj::ObjectBuilder, Target::*, IR::Module};

pub(crate) fn wasm_emit_mccode(registry: &mut TargetRegistry, debug: bool, module: &mut Module) -> Result<(ObjectBuilder, Option<DebugRegistry>), Box<dyn Error>> {
    if debug {
        dbg!("wasm doesn't currently support debugging information so it will be left out");
    }
    
    let mut encoded = Vec::new();

    todo!();

    let mut obj = ObjectBuilder::new(Triple::host()); // triple is unimportant here

    obj.just_write_bytes = Some(encoded);

    Ok((obj, None))
}