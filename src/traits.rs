use std::ops::Index;

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
        Self: Sized,
        for<'a> &'a mut Self: IntoIterator<Item = &'a mut usize>,
        A: Alg<Self>,
        A: Index<usize, Output = usize>,
    {
        for e in self {
            *e = algs[index][*e];
        }
    }
    fn is_solved(&self) -> bool;
}
