use std::ops::Index;

pub trait Puzzle {
    type Move;

    fn apply(&mut self, m: Self::Move)
    where
        Self: Sized;
    fn is_solved(&self) -> bool;
}

pub trait LL {
    type Alg;

    fn apply(&mut self, algs: &[Self::Alg], index: usize)
    where
        Self: Sized,
        for<'a> &'a mut Self: IntoIterator<Item = &'a mut usize>,
        Self::Alg: Index<usize, Output = usize>;

    fn is_solved(&self) -> bool;
}
