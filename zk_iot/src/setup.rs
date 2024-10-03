use crate::{
    dsp_vec, math::*, to_bint
};

use anyhow::Result;

pub struct Setup {
    pub number_gate: u64,
    pub number_output: u64,
    pub number_input: u64,
    pub generator: u64,
    pub random_tau: u64,
    pub random_b: u64,
    pub set_h_len: u64,
    pub set_k_len: u64,
    pub numebr_t_zero: u64,
}

impl Setup {
    pub fn new() -> Self {
        // Initialize
        let ng = 3; // Number of gates
        let no = 1; // Number of outputs
        let ni = 1; // Number of inputs (registers)
        let g  = 2; // Generator number

        // TODO: Define a random value b within the range F(0..P-n) and ensure 0 < b <= P - n
        // let b = thread_rng().gen_range(0..50);
        let b = 2;
        let tau = 119;

        let set_h_len: u64 = ng + ni + 1;
        let numebr_t_zero: u64 = ni + 1;
        let set_k_len = ((set_h_len * set_h_len - set_h_len) / 2)
            - ((numebr_t_zero * numebr_t_zero - numebr_t_zero) / 2);

        Self {
            number_gate: ng,
            number_output: no,
            number_input: ni,
            generator: g,
            random_tau: tau,
            random_b: b,
            set_h_len,
            set_k_len,
            numebr_t_zero,
        }
    }

    pub fn key_generate(&self) -> (Vec<Mfp>, Mfp) {
        let b = self.random_b;
        let n = self.set_h_len;
        let m = self.set_k_len;

        // Calculate each expression
        let expr_vec: Vec<u64> = vec![
            m, self.number_gate - self.number_input + b, n + b, n + 2 * b - 1, 2 * n + b - 1, n + b - 1, n - 1, m - 1, 6 * m - 6, 10_000
        ];
        let max_expr = *expr_vec.iter().max().unwrap();
        let ck = kzg::setup(max_expr, self.random_tau, self.generator);
        let vk = exp_mod(self.generator, self.random_tau);

        (ck, vk)
    }
}
