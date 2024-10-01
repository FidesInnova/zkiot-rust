use crate::{
    math::{exp_mod, Mfp},
    to_bint,
};

pub struct Setup {
    pub number_gate: usize,
    pub number_output: usize,
    pub number_input: usize,
    pub generator: u64,
    pub long_const_val: u64,
    pub base_exp_l: u64,
}

impl Setup {
    pub fn new() -> Self {
        let ng = 3; // Number of gates
        let no = 1; // Number of outputs
        let ni = 1; // Number of inputs (registers)
        let g = 2; // Generator number
        let d = 111213119_u64; // Large constant value
        let l: u64 = 8; // Base exponent for sequence

        Self {
            number_gate: ng,
            number_output: no,
            number_input: ni,
            generator: g,
            long_const_val: d,
            base_exp_l: l,
        }
    }

    pub fn key_generate(&self) -> (Vec<Mfp>, Mfp) {
        
        
        // Calculate each expression
        let expr_vec: Vec<u64> = vec![
            m, ng as u64 - ni as u64 + b, n + b, n + 2 * b - 1, 2 * n + b - 1, n + b - 1, n - 1, m - 1, 6 * m - 6
        ];
        let max_expr = *expr_vec.iter().max().unwrap();
        let ck = kzg::setup(max_expr, tau, g);
        println!("ck: {}", dsp_vec!(ck));
        let vk = exp_mod(g, tau);
    }
}
