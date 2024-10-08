// use crate::a3::ALGNAMES;
// use crate::l3::{CubeAlg, CubeLL};
// use crate::traits::LL;

pub enum Soln {
    One(usize),
    Two(usize, usize)
}
pub type Solns = Vec<Soln>;

// TODO: only does one step, for one case.
// pub fn solve_case(case: &CubeLL, algs: &[CubeAlg]) -> Soln {
//     let mut soln = Soln::new();
//     for index in 0..algs.len() {
//         let mut state: CubeLL = *case;
//         state.apply(algs, index);
//         if state.is_solved() {
//             soln.push(index);
//             break;
//         }
//     }
//     soln
// }


// #[cfg(test)]
// mod test {
//     use crate::l3::A3_I;

//     use super::*;

//     #[test]
//     fn test_solve_one_case_one_alg() {
//         // 1 is at pos 3
//         // 3 is at pos 5
//         // 5 is at pos 1
//         let c = CubeLL([0, 3, 2, 5, 4, 1]);
//         let mut alg: CubeAlg = A3_I; // Start with identity to save writing a full vector.
//         alg.0[3] = 1; // Move pos 3 to pos 1
//         alg.0[5] = 3; // Move pos 5 to pos 3
//         alg.0[1] = 5; // Move pos 1 to pos 5
//         let algs = [A3_I, alg, A3_I];
//         let soln: Soln = solve_case(&c, &algs);
//         assert_eq!(soln, vec![1]);
//     }
// }
