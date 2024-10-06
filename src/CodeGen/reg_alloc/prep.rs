mod alloca;
mod assign;
mod call;
mod cast;
mod cmp;
mod load;
mod math;
mod ret;
mod store;

pub(crate) trait RegAllocPrep<T> {
    fn prep(&mut self, node: &T);
}