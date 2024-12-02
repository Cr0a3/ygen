use std::fmt::Display;

use crate::IR::{Type, TypeMetadata};

use super::{dag::{DagFunction, DagNode, DagOp, DagOpCode}, memory::Memory};

impl Display for DagFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;
        
        for (name, nodes) in &self.blocks {
            writeln!(f, "bb.{}:", name.name)?;

            for node in nodes {
                writeln!(f, "  {node}")?;
            }
        }

        std::fmt::Result::Ok(())
    }
}

impl Display for DagNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(out) = &self.out {
            write!(f, "{out} = ")?;
        }

        write!(f, "{}", self.opcode)?;

        for operand in &self.ops {
            write!(f, " {operand}")?;
        }

        std::fmt::Result::Ok(())
    }
}

impl Display for DagOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.target {
            super::dag::DagOpTarget::Reg(reg) => write!(f, "${reg}")?,
            super::dag::DagOpTarget::UnallocatedVar(var) => write!(f, "{}", var.name)?,
            super::dag::DagOpTarget::Constant(ty) => write!(f, "{} ${}", <Type as Into<TypeMetadata>>::into(*ty), ty.val())?,
            super::dag::DagOpTarget::Mem(mem) => write!(f, "{mem}")?,
        }

        if !self.allocated {
            write!(f, " unalloc")?;
        }
        
        std::fmt::Result::Ok(())
    }
}

impl Display for DagOpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DagOpCode::Copy => write!(f, "copy")?,
            DagOpCode::Ret => write!(f, "ret")?,
            DagOpCode::Add => write!(f, "add")?,
            DagOpCode::And => write!(f, "and")?,
            DagOpCode::Sub => write!(f, "sub")?,
            DagOpCode::Mul => write!(f, "mul")?,
            DagOpCode::Or => write!(f, "or")?,
            DagOpCode::Xor => write!(f, "xor")?,
            DagOpCode::Shr => write!(f, "shr")?,
            DagOpCode::Shl => write!(f, "shl")?,
            DagOpCode::Neg => write!(f, "neg")?,
            DagOpCode::GetFramePtr => write!(f, "intrinsics.getFramePtr")?,
            DagOpCode::GetStackPtr => write!(f, "intrinsics.getStackPtr")?,
            DagOpCode::Br(block) => write!(f, "branch {block}")?,
            DagOpCode::BrIfEq(block) => write!(f, "branch.eq {block}")?,
            DagOpCode::CmpEq => write!(f, "cmp.eq")?,            
            DagOpCode::CmpNe => write!(f, "cmp.ne")?,            
            DagOpCode::CmpLt => write!(f, "cmp.lt")?,            
            DagOpCode::CmpGt => write!(f, "cmp.gt")?,            
            DagOpCode::CmpLte => write!(f, "cmp.lte")?,            
            DagOpCode::CmpGte => write!(f, "cmp.gte")?,        
            DagOpCode::VecInsrt => write!(f, "vec.insrt")?,    

            DagOpCode::I8ToI16 => write!(f, "i8.cast.i16")?,
            DagOpCode::I8ToI32 => write!(f, "i8.cast.i32")?,
            DagOpCode::I8ToI64 => write!(f, "i8.cast.i64")?,
            DagOpCode::I8ToF32 => write!(f, "i8.cast.f32")?,
            DagOpCode::I8ToF64 => write!(f, "i8.cast.f64")?,
            DagOpCode::I16ToI8 => write!(f, "i16.cast.i8")?,
            DagOpCode::I16ToI32 => write!(f, "i16.cast.i32")?,
            DagOpCode::I16ToI64 => write!(f, "i16.cast.i64")?,
            DagOpCode::I16ToF32 => write!(f, "i16.cast.f32")?,
            DagOpCode::I16ToF64 => write!(f, "i16.cast.f64")?,
            DagOpCode::I32ToI8 => write!(f, "i32.cast.i18")?,
            DagOpCode::I32ToI16 => write!(f, "i32.cast.i16")?,
            DagOpCode::I32ToI64 => write!(f, "i32.cast.i64")?,
            DagOpCode::I32ToF32 => write!(f, "i32.cast.f32")?,
            DagOpCode::I32ToF64 => write!(f, "i32.cast.f64")?,
            DagOpCode::I64ToI8 => write!(f, "i64.cast.i8")?,
            DagOpCode::I64ToI16 => write!(f, "i64.cast.i16")?,
            DagOpCode::I64ToI32 => write!(f, "i64.cast.i32")?,
            DagOpCode::I64ToF32 => write!(f, "i64.cast.f32")?,
            DagOpCode::I64ToF64 => write!(f, "i64.cast.f64")?,
            DagOpCode::F32ToI8 => write!(f, "f32.cast.i8")?,
            DagOpCode::F32ToI16 => write!(f, "f32.cast.i16")?,
            DagOpCode::F32ToI32 => write!(f, "f32.cast.i32")?,
            DagOpCode::F32ToI64 => write!(f, "f32.cast.i64")?,
            DagOpCode::F32ToF64 => write!(f, "f32.cast.f64")?,
            DagOpCode::F64ToI8 => write!(f, "f64.cast.i8")?,
            DagOpCode::F64ToI16 => write!(f, "f64.cast.i16")?,
            DagOpCode::F64ToI32 => write!(f, "f64.cast.i32")?,
            DagOpCode::F64ToI64 => write!(f, "f64.cast.i64")?,
            DagOpCode::F64ToF32 => write!(f, "f64.cast.i32")?,
            DagOpCode::Div => write!(f, "div")?,
            DagOpCode::Rem => write!(f, "rem")?,
            DagOpCode::Call(target) => write!(f, "call {target}")?,
        }
        
        std::fmt::Result::Ok(())
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "size_{} ", self.size)?;

        if self.fp_relativ {
            write!(f, "%frame.")?;
        } else if self.sp_relativ {
            write!(f, "%stack.")?;
        }

        write!(f, "{}", self.offset)?;

        std::fmt::Result::Ok(())
    }
}