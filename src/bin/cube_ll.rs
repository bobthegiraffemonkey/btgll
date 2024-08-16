// use btgll::a3::ALGS;
use btgll::cube::ll::CubeLL;
use btgll::solve::solve_case;

// TODO: gen actual cases.
const CASES: &[CubeLL] = &[CubeLL([0, 1, 2, 5, 4, 7])]; // U-perm, soln UF->UR->UL

fn main() {
    let soln = solve_case(&CASES[0], ALGS);
    // print_soln(soln);
}
