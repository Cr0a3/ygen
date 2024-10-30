use crate::CodeGen::{Allocator, MachineCallingConvention, Reg, VarLocation};
use crate::prelude::{Alloca, Function, Ir, Phi, TypeMetadata};
use crate::Target::Arch;
use crate::IR::Var;

fn arg_prep(alloc: &mut Allocator, func: &Function, call: MachineCallingConvention) {
    let mut index = 0;

    for (name, ty) in &func.ty.args {
        if let Some(reg) = call.arg(Arch::X86_64, *ty, index) {
            // argument in an register

            alloc.vars.insert(name.to_owned(), VarLocation::Reg(reg));
            alloc.var_types.insert(name.to_owned(), *ty);

            let mut removeal_index = 0;

            for freg in alloc.fregs.clone() {
                if freg.is(&reg) {
                    alloc.fregs.remove(removeal_index);
                    break;
                }

                removeal_index += 1;
            }
        } else {
            // argument on stack
            todo!("x64 currently doesn't support arguments which are passed over the stack");
        }

        index += 1;
    }
}

pub(crate) fn x64_alloc(alloc: &mut Allocator, func: &Function) {
    arg_prep(alloc, func, alloc.call);

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
    let inputs = node.inputs();

    for _input in inputs {
        // potential freeing here
    }

    let mut scopes = Vec::new();

    for (name, location) in &alloc.vars {
        scopes.push( (Var {
            name: name.to_owned(),
            ty: *alloc.var_types.get(name).unwrap(),
        }, *location) );
    }

    alloc.scopes.insert(node.dump(), scopes);

    // handle specific nodes here (like alloca)
    if let Some(alloca) = node.as_any().downcast_ref::<Alloca<Var, TypeMetadata>>() {
        let location = x64_alloc_stack(alloc, alloca.inner2);

        alloc.vars.insert(alloca.inner1.name.to_owned(), location);
        alloc.var_types.insert(alloca.inner1.name.to_owned(), alloca.inner2);

        alloc.allocated_vars.push(alloca.inner1.name.to_owned());
        return;
    }

    if let Some(_) = node.as_any().downcast_ref::<Phi>() { // phis were handled before
        return;
    }

    if let Some(out) = node.output() {
        let location = x64_alloc_rv(alloc, out.ty);

        alloc.vars.insert(out.name.to_owned(), location);
        alloc.var_types.insert(out.name.to_owned(), out.ty);
    }
}

pub(crate) fn x64_alloc_rv(alloc: &mut Allocator, ty: TypeMetadata) -> VarLocation {
    let vec = if ty.float() { &mut alloc.ffpregs } else { &mut alloc.fregs }; // select free registers vec

    if let Some(reg) = vec.pop() {
        VarLocation::Reg(match reg { // this code here just changes the register to be the fitting type 
            Reg::x64(x64) => Reg::x64(x64.sub_ty(ty)),
            _ => panic!("the x64 register allocator just got an non x64 register")
        })
    } else {
        x64_alloc_stack(alloc, ty)
    }
}

pub(crate) fn x64_alloc_stack(alloc: &mut Allocator, ty: TypeMetadata) -> VarLocation {
    alloc.epilog = true;

    let ret = VarLocation::Mem(alloc.stack_off, ty);    
    alloc.stack_off += 8; // alignment
    
    ret
}

pub(crate) fn phi_prep(alloc: &mut Allocator, phi: &Phi) {
    let out = x64_alloc_rv(alloc, phi.typ);

    for (_, var) in &phi.recive_from_blocks {
        alloc.phi_vars.insert(var.name.to_owned(), out);
    }

    alloc.vars.insert(phi.out.name.to_owned(), out);
    alloc.var_types.insert(phi.out.name.to_owned(), phi.typ);
}

pub(crate) fn x64_free(alloc: &mut Allocator, loc: VarLocation) {
    if let VarLocation::Reg(reg) = loc {
        if reg.is_gr() {
            alloc.fregs.push(reg);
        } else if reg.is_fp() {
            alloc.ffpregs.push(reg);
        }
    }
}