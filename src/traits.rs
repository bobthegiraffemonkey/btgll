pub trait Move<P> {}

pub trait Puzzle<M> {
    fn apply(&mut self, m: M)
    where
        M: Move<Self>,
        Self: Sized;
    fn is_solved(&self) -> bool;
}

pub trait Alg<LL> {}
pub trait LL<A> {
    fn apply(&mut self, algs: &[A], index: usize)
    where
        A: Alg<Self>,
        Self: Sized;
    fn is_solved(&self) -> bool;
}
