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
            DagOpCode::GetFramePtr => write!(f, "intrinsics.getFramePtr")?,
            DagOpCode::GetStackPtr => write!(f, "intrinsics.getStackPtr")?,
            
        }
        
        std::fmt::Result::Ok(())
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "size_{}", self.size)?;

        if self.fp_relativ {
            write!(f, "%frame.")?;
        } else if self.sp_relativ {
            write!(f, "%stack.")?;
        }

        write!(f, "{}", self.offset)?;

        std::fmt::Result::Ok(())
    }
}