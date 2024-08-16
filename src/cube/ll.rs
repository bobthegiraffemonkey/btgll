//     8  9 10
// 19  0  1  2 11
// 18  7     3 12
// 17  6  5  4 13
//    16 15 14

use crate::traits::LL;
use btgll_derive::LL;

// Names must match what LL derive expects.
const CUBELL_SIZE: usize = 6; // (edges - 1) + (corners - 1)
const CUBELL_ALG_SIZE: usize = 20; // edge_stickers + corner_stickers

#[derive(PartialEq, LL)]
pub struct CubeLL(pub [usize; CUBELL_SIZE]);

pub const A3_I: CubeLLAlg = CubeLLAlg([
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
]);

#[cfg(test)]
mod test {
    use super::*;

    const U_PERM: CubeLLAlg = CubeLLAlg([
        0, 3, 2, 5, 4, 1, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ]);
    const TEST_ALGS: &[CubeLLAlg] = &[U_PERM];

    #[test]
    fn test_algs() {
        let mut ll = CUBELL_SOLVED;
        for _ in 0..=2 {
            ll.apply(TEST_ALGS, 0);
        }
        assert!(ll.is_solved());
    }
}
