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

    pub fn proof_path(&self) -> Vec<Mfp> {
        // Generate the proof path by iteratively applying exponentiation
        let mut proof_path = vec![];
        let mut s = Mfp::from(self.generator);
        let d = Mfp::from(self.long_const_val);
        for _ in 0..=self.base_exp_l {
            proof_path.push(s);
            s = exp_mod(to_bint!(s), to_bint!(d));
        }

        proof_path
    }
}
