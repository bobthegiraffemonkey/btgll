// //            0  1  2
// //            7     3
// //            6  5  4
// //
// // 19 18 17  16 15 14  13 12 11  10  9  8
// // 27    26  25    24  23    22  21    20
// // 39 38 37  36 35 34  33 32 31  30 29 28
// //
// //           40 41 42
// //           47    43
// //           46 45 44

// use crate::traits::Puzzle;

// type CubeInner = [usize; 18]; // (edges - 1) + (corners - 1)
// type CubeTurnInner = [usize; 48]; // edge_stickers + corner_stickers

// // #[derive(LL)]
// pub struct Cube([usize; 18]);
// pub struct CubeTurn([usize; 48]);

// pub const C3_SOLVED: Cube = Cube([
//     0, 1, 2, 3, 4, 5, 6, 7, 20, 21, 24, 25, 40, 41, 42, 43, 44, 45,
// ]);

// #[derive(Clone, Copy)]
// pub enum CubeMove {
//     U,
//     D,
//     R,
//     L,
//     F,
//     B,
// }
// pub use CubeMove::*;

// pub const T3_I: CubeTurn = CubeTurn([
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
//     26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
// ]);

// const TURN_U: CubeTurn = CubeTurn([
//     1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
//     26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
// ]);
// const TURN_D: CubeTurn = CubeTurn([
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
//     26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
// ]);
// const TURN_F: CubeTurn = CubeTurn([
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
//     26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
// ]);
// const TURN_B: CubeTurn = CubeTurn([
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
//     26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
// ]);
// const TURN_R: CubeTurn = CubeTurn([
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
//     26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
// ]);
// const TURN_L: CubeTurn = CubeTurn([
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
//     26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
// ]);

// impl Puzzle for Cube {
//     type Move = CubeMove;

//     fn apply(&mut self, m: CubeMove) {
//         let map = match m {
//             CubeMove::U => TURN_U,
//             CubeMove::D => TURN_D,
//             CubeMove::F => TURN_F,
//             CubeMove::B => TURN_B,
//             CubeMove::R => TURN_R,
//             CubeMove::L => TURN_L,
//         };
//         for e in self.0.iter_mut() {
//             *e = map[*e];
//         }
//     }

//     fn is_solved(&self) -> bool {
//         *self == C3_SOLVED
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_turns() {
//         // assert_ne!(T3_I, TURN_U);
//         // assert_ne!(T3_I, TURN_D);
//         // assert_ne!(T3_I, TURN_F);
//         // assert_ne!(T3_I, TURN_B);
//         // assert_ne!(T3_I, TURN_R);
//         // assert_ne!(T3_I, TURN_L);

//         let mut c = C3_SOLVED;
//         for test_move in [U, D, F, B, R, L] {
//             for _ in 0..=3 {
//                 c.apply(test_move)
//             }
//             assert!(c.is_solved());
//         }
//     }
// }
