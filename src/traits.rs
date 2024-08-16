pub trait Puzzle {
    type Move;

    fn apply(&mut self, m: Self::Move);
    fn is_solved(&self) -> bool;
}

pub trait LL {
    type Alg;

    fn apply(&mut self, algs: &[Self::Alg], index: usize);
    fn is_solved(&self) -> bool;
}
