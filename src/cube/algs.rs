use super::ll::CubeLLAlg;

pub const ALGS: &[CubeLLAlg] = &[CubeLLAlg([
    0, 1, 2, 7, 4, 3, 6, 5, 8, 9, 10, 11, 18, 13, 14, 12, 16, 17, 15, 19, // UF->UR->UL
])];

pub const ALGNAMES: [&str; 1] = ["U"];
