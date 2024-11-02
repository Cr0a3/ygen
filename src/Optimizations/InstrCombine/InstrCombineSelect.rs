use crate::Optimizations::Passes::InstrCombinePass;
use crate::IR::ir::*;

fn optimizeSelectToCast<T, U>(SI: &Select<T, U>) -> Option<Box<dyn Ir>> where
    T: std::fmt::Debug + Clone + PartialEq + Eq + crate::Support::AsAny + 'static,
    U: std::fmt::Debug + Clone + PartialEq + Eq + crate::Support::AsAny + 'static
{
    if !(SI.isTrueConst() && SI.isFalseConst()) {
        return None;
    }

    let TrueVal = SI.getTrueConst();
    let FalseVal = SI.getFalseConst();

    if !(TrueVal.val() == 1.0 && FalseVal.val() == 0.0) {
        return None;
    }


    Some(Cast::new(SI.getCondition(), SI.getSelType(), SI.getOut()))
}

impl InstrCombinePass {
    /// Tries to optimize an select node
    pub(crate) fn opt_select<T, U>(SI: &Select<T, U>) -> Option<Box<dyn Ir>> where
        T: std::fmt::Debug + Clone + PartialEq + Eq + crate::Support::AsAny + 'static,
        U: std::fmt::Debug + Clone + PartialEq + Eq + crate::Support::AsAny + 'static
    {
        if let Some(opt) = optimizeSelectToCast(SI) { return Some(opt) };

        None
    }
}