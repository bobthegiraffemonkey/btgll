//     8  9 10
// 19  0  1  2 11
// 18  7     3 12
// 17  6  5  4 13
//    16 15 14

use crate::traits::LL;
use btgll_derive::LL;

const CUBELL_SIZE: usize = 6; // (edges - 1) + (corners - 1)
const CUBELLALG_SIZE: usize = 20; // edge_stickers + corner_stickers

#[derive(PartialEq)]
pub struct CubeLL(pub [usize; CUBELL_SIZE]);

impl std::ops::Deref for CubeLL {
    type Target = [usize; CUBELL_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CubeLL {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct CubeLLAlg([usize; CUBELLALG_SIZE]); const CUBELL_SOLVED : CubeLL =
           {
               let mut array = [0; CUBELL_SIZE]; let mut index = 0; while index <
               CUBELL_SIZE { array [index] = index; index += 1; } CubeLL(array)
           }; impl LL for CubeLL
           {
               type Alg = CubeLLAlg; fn
               apply(& mut self, algs : & [Self :: Alg], index : usize)
               { for ref mut e in **self { * e = algs [index] [*e]; } } fn is_solved(& self) ->
               bool { * self == CUBELL_SOLVED }
           } impl std :: ops :: Index < usize > for CubeLLAlg
           {
               type Output = usize; fn index(& self, index : usize) -> & Self :: Output
               { & self.0 [index] }
           }
// impl IntoIterator for CubeLL {
//     type Item = usize;

//     type IntoIter = std::array::IntoIter<Self::Item, CUBE_LL_SIZE>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

// impl Index<usize> for CubeLL {
//     type Output = usize;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.0[index]
//     }
// }

// pub struct CubeAlg();

// impl Index<usize> for CubeAlg {
//     type Output = usize;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.0[index]
//     }
// }

pub const L3_SOLVED: CubeLL = CubeLL([0, 1, 2, 3, 4, 5]);
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
        let mut ll = L3_SOLVED;
        for _ in 0..=2 {
            ll.apply(TEST_ALGS, 0);
        }
        assert!(ll.is_solved());
    }
}
