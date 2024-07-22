//     8  9 10
// 19  0  1  2 11
// 18  7     3 12
// 17  6  5  4 13
//    16 15 14

use crate::traits::{Alg, LL};

pub type L3 = [usize; 6]; // (edges - 1) + (corners - 1)
pub type A3 = [usize; 20]; // edge_stickers + corner_stickers

pub const L3_SOLVED: L3 = [0, 1, 2, 3, 4, 5];
pub const A3_I: A3 = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
];

impl LL<A3> for L3 {
    fn apply(&mut self, algs: &[A3], index: usize) {
        for e in self.iter_mut() {
            *e = algs[index][*e]; // wrong?
        }
    }

    fn is_solved(&self) -> bool {
        *self == L3_SOLVED
    }
}

impl Alg<L3> for A3 {}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_ALGS: &[A3] = &[[
        0, 3, 2, 5, 4, 1, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ]];

    #[test]
    fn test_algs() {
        let mut ll = L3_SOLVED;
        for _ in 0..=2 {
            ll.apply(TEST_ALGS, 0);
        }
        assert!(ll.is_solved());
    }
}
