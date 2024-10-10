mod alloca;
mod assign;
mod call;
mod cast;
mod cmp;
mod load;
mod math;
mod neg;
mod ret;
mod select;
mod store;
mod extractelement;

pub(crate) trait RegAllocPrep<T> {
    fn prep(&mut self, node: &T);
}