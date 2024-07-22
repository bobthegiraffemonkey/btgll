use btgll::a3::ALGS;
use btgll::l3::L3;
use btgll::solve::{print_soln, solve_case};

// TODO: gen actual cases.
const CASES: &[L3] = &[[0, 1, 2, 5, 4, 7]]; // U-perm, soln UF->UR->UL

fn main() {
    let soln = solve_case(&CASES[0], ALGS);
    print_soln(soln);
}
