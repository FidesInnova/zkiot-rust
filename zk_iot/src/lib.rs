// incomplete ZKP scheme with "nalgebra" lib
// initializes and partially fills matrices A, B, C

use nalgebra::DMatrix;


#[derive(Clone, Copy)]
enum GateType {
    Addition,
    Multiplication,
}

struct Gate {
    left: usize,
    right: usize,
    gate_type: GateType,
}


pub struct ZKPScheme {
    pub a: DMatrix<u32>,
    pub b: DMatrix<u32>,
    pub c: DMatrix<u32>,
    pub ng: usize,
    pub no: usize,
}

impl ZKPScheme {
    pub fn new(ng: usize, no: usize) -> Self {
        let size = ng + no + 1;
        let a = DMatrix::zeros(size, size);
        let b = DMatrix::zeros(size, size);
        let c = DMatrix::zeros(size, size);

        ZKPScheme { a, b, c, ng, no }
    }
}