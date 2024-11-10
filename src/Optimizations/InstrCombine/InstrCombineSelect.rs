use crate::Optimizations::Passes::InstrCombinePass;
use crate::IR::ir::*;

fn optimizeSelectToCast(SI: &Select) -> Option<Box<dyn Ir>> {
    if !(SI.isTrueConst() && SI.isFalseConst()) {
        return None;
    }

    let TrueVal = SI.getTrueConst();
    let FalseVal = SI.getFalseConst();

    if !(TrueVal.val() == 1.0 && FalseVal.val() == 0.0) {
        return None;
    }


    Some(Cast::new(IROperand::Var(SI.getCondition()), SI.getSelType(), SI.getOut()))
}

impl InstrCombinePass {
    /// Tries to optimize an select node
    pub(crate) fn opt_select(SI: &Select) -> Option<Box<dyn Ir>> {
        if let Some(opt) = optimizeSelectToCast(SI) { return Some(opt) };

        None
    }
}