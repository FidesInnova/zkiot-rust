use crate::{
    math::{commit, exp_mod, generate_set, Mfp},
    setup::Setup,
    to_bint,
    utils::{encode_matrix_m, init, rows_to_zero, Gate},
};
use anyhow::Result;
use ark_ff::{Field, PrimeField};
use nalgebra::DMatrix;

pub struct Commitment {
    pub set_h_len: u64,
    pub set_k_len: u64,
    pub set_h: Vec<Mfp>,
    pub set_k: Vec<Mfp>,
    pub numebr_t_zero: usize,
    pub matrices: Matrices,
}

impl Commitment {
    // Constructor method Generate sets and Initilize matrices
    pub fn new(setup: &Setup) -> Result<Self> {
        let set_h_len: u64 = (setup.number_gate + setup.number_input + 1).try_into()?;
        let numebr_t_zero: u64 = (setup.number_input + 1).try_into()?; // Number of rows (|x| = t, where t = ni + 1)
        let set_k_len = ((set_h_len * set_h_len - set_h_len) / 2)
            - ((numebr_t_zero * numebr_t_zero - numebr_t_zero) / 2);

        let generator_h = to_bint!(exp_mod(
            setup.generator,
            (Mfp::MODULUS.0[0] - 1) / set_h_len
        )); // Compute the generator for set H
        let generator_k = to_bint!(exp_mod(
            setup.generator,
            (Mfp::MODULUS.0[0] - 1) / set_k_len 
        )); // Compute the generator for set K

        let set_h = generate_set(generator_h, set_h_len);
        let set_k = generate_set(generator_k, set_k_len);

        let matrix_size = setup.number_gate + setup.number_input + 1;
        let matrices = Matrices::new(matrix_size);

        Ok(Self {
            set_h_len,
            set_k_len,
            numebr_t_zero: numebr_t_zero.try_into()?,
            set_h,
            set_k,
            matrices,
        })
    }

    // Construction of matrices based on the algorithm for initializing matrices during the Commitment Phase
    pub fn build_matrices(&mut self, gates: Vec<Gate>, number_gate: usize) {
        // Initialize matrices A, B, C and z based on parsed gates
        init(
            gates,
            number_gate,
            &mut self.matrices.a,
            &mut self.matrices.b,
            &mut self.matrices.c,
            &mut self.matrices.z,
        );

        // Set specific rows in matrices A, B, C to zero
        rows_to_zero(&mut self.matrices.a, self.numebr_t_zero);
        rows_to_zero(&mut self.matrices.b, self.numebr_t_zero);
        rows_to_zero(&mut self.matrices.c, self.numebr_t_zero);
    }

    pub fn commit(&self, long_const_val: u64, generator: u64) -> Vec<Mfp> {
        // A matrix processing
        let a_matrix_encode = encode_matrix_m(&self.matrices.a, &self.set_h, &self.set_k);

        // // B matrix processing
        let b_matrix_encode = encode_matrix_m(&self.matrices.b, &self.set_h, &self.set_k);

        // // C matrix processing
        let c_matrix_encode = encode_matrix_m(&self.matrices.c, &self.set_h, &self.set_k);

        // // Combine encoded matrix polynomials
        let mut o_i = vec![];

        // Append encoded matrices
        o_i.extend(a_matrix_encode); // Add encoded polynomials for matrix A
        o_i.extend(b_matrix_encode); // Add encoded polynomials for matrix B
        o_i.extend(c_matrix_encode); // Add encoded polynomials for matrix C

        commit(&o_i, long_const_val, generator)               // Generate the commitment
    }
}

pub struct Matrices {
    pub a: DMatrix<Mfp>,
    pub b: DMatrix<Mfp>,
    pub c: DMatrix<Mfp>,
    pub z: DMatrix<Mfp>,
    pub size: usize,
}

impl Matrices {
    pub fn new(size: usize) -> Self {
        let a = DMatrix::<Mfp>::zeros(size, size);
        let b = DMatrix::<Mfp>::zeros(size, size);
        let c = DMatrix::<Mfp>::zeros(size, size);
        let mut z = DMatrix::<Mfp>::zeros(size, 1);
        z[0] = Mfp::ONE;

        Self { a, b, c, z, size }
    }
}
