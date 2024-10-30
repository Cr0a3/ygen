use crate::CodeGen::{Allocator, Reg, VarLocation};
use crate::prelude::{Alloca, Function, Ir, Phi, TypeMetadata};
use crate::IR::Var;

fn arg_prep(alloc: &mut Allocator, func: &Function) {
    // TODO: maybe replace index variable with stack_off?
    
    let mut index = 0;
    for (name, ty) in &func.ty.args {
        alloc.vars.insert(name.to_owned(), VarLocation::Reg(Reg::wasm(index, *ty)));
        alloc.var_types.insert(name.to_owned(), *ty);

        alloc.stack_off += 1; // the stack off is in reality really only the amount of current variables

        index += 1;
    }
}

pub(crate) fn wasm_alloc(alloc: &mut Allocator, func: &Function) {
    arg_prep(alloc, func);

    // run phis
    for block in &func.blocks {
        for node in &block.nodes {
            if let Some(phi) = node.as_any().downcast_ref::<Phi>() {
                phi_prep( alloc, phi );
            }
        }
    }

    for block in &func.blocks {
        for node in &block.nodes {
            node_prep(alloc, node);
        }
    }
}

fn node_prep(alloc: &mut Allocator, node: &Box<dyn Ir>) {
    let mut scopes = Vec::new();
    
    for (name, location) in &alloc.vars {
        scopes.push( (Var {
            name: name.to_owned(),
            ty: *alloc.var_types.get(name).unwrap(),
        }, *location) );
    }
    
    alloc.scopes.insert(node.dump(), scopes);
    
    let inputs = node.inputs();
    for _input in inputs {
        // potential freeing here
    }

    // handle specific nodes here (like alloca)
    if let Some(alloca) = node.as_any().downcast_ref::<Alloca<Var, TypeMetadata>>() {
        let location = wasm_alloc_var(alloc, alloca.inner2);

        alloc.vars.insert(alloca.inner1.name.to_owned(), location);
        alloc.var_types.insert(alloca.inner1.name.to_owned(), alloca.inner2);

        alloc.allocated_vars.push(alloca.inner1.name.to_owned());
        return;
    }

    if let Some(_) = node.as_any().downcast_ref::<Phi>() { // phis were handled before
        return;
    }

    if let Some(out) = node.output() {
        let location = wasm_alloc_var(alloc, out.ty);

        alloc.vars.insert(out.name.to_owned(), location);
        alloc.var_types.insert(out.name.to_owned(), out.ty);
    }
}

pub(crate) fn wasm_alloc_var(alloc: &mut Allocator, ty: TypeMetadata) -> VarLocation {
    if let Some(reg) = alloc.fregs.pop() {
        debug_assert!(matches!(reg, Reg::wasm(_, _)));
        let Reg::wasm(_, reg_ty) = reg else { 
            panic!("the wasm backend expects every variable location to be an wasm register");
        };
        
        if reg_ty == ty {
            return VarLocation::Reg(reg);
        }
    }

    alloc.stack_off += 1; // the stack off is in reality really only the amount of current variables

    VarLocation::Reg(Reg::wasm((alloc.stack_off - 1) as i32, ty))
}

pub(crate) fn phi_prep(alloc: &mut Allocator, phi: &Phi) {
    let out = wasm_alloc_var(alloc, phi.typ);

    for (_, var) in &phi.recive_from_blocks {
        alloc.phi_vars.insert(var.name.to_owned(), out);
    }

    alloc.vars.insert(phi.out.name.to_owned(), out);
    alloc.var_types.insert(phi.out.name.to_owned(), phi.typ);
}

pub(crate) fn wasm_free(alloc: &mut Allocator, loc: VarLocation) {
    let VarLocation::Reg(Reg::wasm(idx, ty)) = loc else { 
        panic!("the wasm backend expects every variable location to be an wasm register");
    };

   alloc.fregs.push(Reg::wasm(idx, ty));
}