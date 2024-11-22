use std::collections::HashMap;

use crate::ydbg;

use super::*;

/// Basic x86 assembly optimizations
pub struct X86BasicOpt;

impl X86BasicOpt {
    /// Optimizes the given input
    pub fn opt(input: &mut Vec<X86Instr>) {
        let is_mov = |instr: X86Instr| {
            matches!(instr.mnemonic, X86Mnemonic::Mov | X86Mnemonic::Movdqa | X86Mnemonic::Movsd | X86Mnemonic::Movss)
        };

        let mut index = 0;
        X86BasicOpt::inline_regs(input);

        for instr in input.clone() {
            let mut removed = false;

            // mov x, x -> nothing
            if is_mov(instr) && instr.op1 == instr.op2 {
                ydbg!("[X86 OPT] removing uneccesary mov");
                input.remove(index);
                removed = true;
            }
            
            if !removed {
                index += 1;
            }
        }
    }

    /// Inlines some registers for the given assembly code
    pub fn inline_regs(input: &mut Vec<X86Instr>) {
        let is_mov = |instr: &X86Instr| {
            matches!(instr.mnemonic, X86Mnemonic::Mov | X86Mnemonic::Movdqa | X86Mnemonic::Movsd | X86Mnemonic::Movss)
        };

        // How it works:
        // 1. It creates a list of all temporarys and their value
        // 2. It checks if the location of the original value gets overriten
        //  |-> If not it removes the temporary and directly uses its value

        // This here is the "gathering" step

        let mut tmps = HashMap::new();
        let mut original_tmp_assignments = HashMap::new();
        let mut required_tmps = Vec::new();

        let mut index = 0;

        for instr in input.iter() {
            if is_mov(instr) {
                if let Some(X86Operand::Tmp(tmp)) = instr.op1 {
                    if !tmps.contains_key(&tmp) {
                        original_tmp_assignments.insert(tmp, index);
                    } else { panic!("tmps already contains key {tmp}"); }
                    tmps.insert(tmp, instr.op2.expect("mov expects second operand"));
                }
            }

            if let Some(op) = &instr.op1 {
                for (tmp, value) in &tmps {
                    if op == value {
                        required_tmps.push(*tmp);
                    }
                }
            }

            index += 1;
        }

        // Then we now have the transformation step

        let mut index = 0;
        let mut vindex = 0;

        let mut to_remove = Vec::new();

        for instr in input.iter_mut() {
            let mut marked_as_remove = false;

            for (tmp, value) in &tmps {
                if required_tmps.contains(tmp) { continue; }

                for (_, key) in &original_tmp_assignments {
                    if *key == vindex {
                        marked_as_remove = true;
                    }
                }

                if let Some(X86Operand::Tmp(ref used_tmp)) = instr.op2 {
                    if tmp == used_tmp {
                        instr.op2 = Some(value.to_owned())
                    } 
                }
                if let Some(X86Operand::Tmp(ref used_tmp)) = instr.op3 {
                    if tmp == used_tmp {
                        instr.op3 = Some(value.to_owned())
                    } 
                }
            }
            
            if marked_as_remove { to_remove.push(index); }
            else { index += 1; }

            vindex += 1;
        }

        // now finally we remove the unrequired assignments

        for idx in to_remove {
            input.remove(idx);
        }
    }
}