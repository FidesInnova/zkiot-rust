
use std::collections::HashMap;

use crate::{
    dsp_mat, dsp_poly, dsp_vec, json_file::store_commit_json, math::*, setup::*, to_bint, utils::*,
};
use anyhow::Result;
use ark_ff::{Field, PrimeField};
use nalgebra::DMatrix;
use rustnomial::Evaluable;


#[derive(Debug)]
pub enum Component {
    Row,
    Col,
    Val,
}

#[derive(Debug)]
pub enum EntityType {
    Polynomial(Component ,Lable),
    Points(Component ,Lable),
}

#[derive(Debug)]
pub enum Lable {
    A,
    B,
    C,
}

#[derive(Debug)]
pub enum DataType {
    Polynomial(Poly),
    Points(HashMap<Mfp, Mfp>)
}

#[derive(Debug, Clone)]
pub struct Commitment {
    pub set_h: Vec<Mfp>,
    pub set_k: Vec<Mfp>,
    pub numebr_t_zero: usize,
    pub matrices: Matrices,
    pub polys_px: Vec<Poly>,
    pub points_px: Vec<HashMap<Mfp, Mfp>>,
}

impl Commitment {
    // Constructor method Generate sets and Initilize matrices
    pub fn new(setup: &Setup) -> CommitmentBuilder {
        let set_h_len: u64 = (setup.number_gate + setup.number_input + 1)
            .try_into()
            .unwrap();
        let numebr_t_zero: u64 = (setup.number_input + 1).try_into().unwrap(); // Number of rows (|x| = self.numebr_t_zero, where self.numebr_t_zero = ni + 1)
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
        let matrices = Matrices::new(matrix_size.try_into().unwrap());

        CommitmentBuilder {
            commitm: Commitment {
                set_h,
                set_k,
                numebr_t_zero: numebr_t_zero.try_into().unwrap(),
                matrices,
                polys_px: vec![],
                points_px: vec![],
            },
        }
    }

    /// Generates a commitment based on the AHP commitment generation process.
    /// For more details, see:
    /// [AHP Commitment Generation Documentation](https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/2-commitment-phase#id-2-3-ahp-commitment)
    pub fn get_polynomials_commitment(
        &self,
        generator: u64,
        commitment_key: &Vec<Mfp>,
    ) -> Vec<Mfp> {
        // let o_ahp = concat_polys(&polys_pxs);
        // println!("o_ahp: {}", dsp_vec!(o_ahp));

        let commitment = compute_all_commitment(&self.polys_px, commitment_key, generator);
        println!("com_ahp: {}", dsp_vec!(commitment));

        commitment
    }

    pub fn get_matrix_az(&self) -> DMatrix<Mfp> {
        &self.matrices.a * &self.matrices.z
    }
    pub fn get_matrix_bz(&self) -> DMatrix<Mfp> {
        &self.matrices.b * &self.matrices.z
    }
    pub fn get_matrix_cz(&self) -> DMatrix<Mfp> {
        &self.matrices.c * &self.matrices.z
    }

    /// Store in Json file
    fn store(&self, path: &str) -> Result<()> {
        todo!()
    }

    /// Restore Commitment from Json file
    fn restore(path: &str) -> Result<Self> {
        todo!()
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct CommitmentBuilder {
    commitm: Commitment,
}

impl CommitmentBuilder {
    // Construction of matrices based on the algorithm for initializing matrices during the Commitment Phase
    pub fn build_matrices(&mut self, gates: Vec<Gate>, number_inputs: usize) -> Self {
        // Initialize matrices A, B, C and z based on parsed gates
        build_matrices(
            gates,
            number_inputs,
            &mut self.commitm.matrices.a,
            &mut self.commitm.matrices.b,
            &mut self.commitm.matrices.c,
            &mut self.commitm.matrices.z,
        );

        // Set specific rows in matrices A, B, C to zero
        rows_to_zero(&mut self.commitm.matrices.a, self.commitm.numebr_t_zero);
        rows_to_zero(&mut self.commitm.matrices.b, self.commitm.numebr_t_zero);
        rows_to_zero(&mut self.commitm.matrices.c, self.commitm.numebr_t_zero);

        self.clone()
    }

    pub fn build_polynomials(&mut self) -> Self {
        // TODO:
        // Uncomment and adjust the line below to push random points
        // push_random_points(&mut points_za, b, &vec_to_set(&self.commitm.set_h));
        // push_random_points(&mut points_zb, b, &vec_to_set(&self.commitm.set_h));
        // push_random_points(&mut points_zc, b, &vec_to_set(&self.commitm.set_h));
        // Matrix A:
        // From wiki: (https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/2-commitment-phase#id-2-5-2-ahp-commitment)
        let mut points_row_p_a = get_matrix_point_row(
            &self.commitm.matrices.a,
            &self.commitm.set_h,
            &self.commitm.set_k,
        );
        // rowA' = (48, 1), (73, 135), (62, 125), (132, 59), (65, 42), (80, 1)
        points_row_p_a.insert(Mfp::from(48), Mfp::from(1));
        points_row_p_a.insert(Mfp::from(73), Mfp::from(135));
        points_row_p_a.insert(Mfp::from(62), Mfp::from(125));
        points_row_p_a.insert(Mfp::from(132), Mfp::from(59));
        points_row_p_a.insert(Mfp::from(65), Mfp::from(42));
        points_row_p_a.insert(Mfp::from(80), Mfp::from(1));

        let mut points_col_p_a = get_matrix_point_col(
            &self.commitm.matrices.a,
            &self.commitm.set_h,
            &self.commitm.set_k,
        );
        // colA' = (48, 42), (73, 1), (62, 135), (132, 125), (65, 59), (80, 42)
        points_col_p_a.insert(Mfp::from(48), Mfp::from(42));
        points_col_p_a.insert(Mfp::from(73), Mfp::from(1));
        points_col_p_a.insert(Mfp::from(62), Mfp::from(135));
        points_col_p_a.insert(Mfp::from(132), Mfp::from(125));
        points_col_p_a.insert(Mfp::from(65), Mfp::from(59));
        points_col_p_a.insert(Mfp::from(80), Mfp::from(42));

        let points_val_p_a = get_matrix_point_val(
            &self.commitm.matrices.a,
            &self.commitm.set_h,
            &self.commitm.set_k,
            &points_row_p_a,
            &points_col_p_a,
        );

        // Matrix B:
        let mut points_row_p_b = get_matrix_point_row(
            &self.commitm.matrices.b,
            &self.commitm.set_h,
            &self.commitm.set_k,
        );
        // rowB' = (73, 59), (62, 1), (132, 42), (65, 135), (80, 59)
        points_row_p_b.insert(Mfp::from(73), Mfp::from(59));
        points_row_p_b.insert(Mfp::from(62), Mfp::from(1));
        points_row_p_b.insert(Mfp::from(132), Mfp::from(42));
        points_row_p_b.insert(Mfp::from(65), Mfp::from(135));
        points_row_p_b.insert(Mfp::from(80), Mfp::from(59));

        let mut points_col_p_b = get_matrix_point_col(
            &self.commitm.matrices.b,
            &self.commitm.set_h,
            &self.commitm.set_k,
        );

        // colB' = (73, 59), (62, 42), (132, 125), (65, 1), (80, 135)
        points_col_p_b.insert(Mfp::from(73), Mfp::from(59));
        points_col_p_b.insert(Mfp::from(62), Mfp::from(42));
        points_col_p_b.insert(Mfp::from(132), Mfp::from(125));
        points_col_p_b.insert(Mfp::from(65), Mfp::from(1));
        points_col_p_b.insert(Mfp::from(80), Mfp::from(135));

        let points_val_p_b = get_matrix_point_val(
            &self.commitm.matrices.b,
            &self.commitm.set_h,
            &self.commitm.set_k,
            &points_row_p_b,
            &points_col_p_b,
        );

        // Matrix C
        let mut points_row_p_c = get_matrix_point_row(
            &self.commitm.matrices.c,
            &self.commitm.set_h,
            &self.commitm.set_k,
        );
        // FIXME: Wiki
        // rowC' = (48, 1), (73, 59), (62, 125), (132, 1), (65, 135), (80, 42)
        points_row_p_c.insert(Mfp::from(48), Mfp::from(1));
        points_row_p_c.insert(Mfp::from(73), Mfp::from(59));
        points_row_p_c.insert(Mfp::from(62), Mfp::from(125));
        points_row_p_c.insert(Mfp::from(132), Mfp::from(1));
        points_row_p_c.insert(Mfp::from(65), Mfp::from(135));
        points_row_p_c.insert(Mfp::from(80), Mfp::from(42));

        let mut points_col_p_c = get_matrix_point_col(
            &self.commitm.matrices.c,
            &self.commitm.set_h,
            &self.commitm.set_k,
        );
        // FIXME: Wiki
        // colC' = (48, 125), (73, 59), (62, 1), (132, 1), (65, 42), (80, 59)
        points_col_p_c.insert(Mfp::from(48), Mfp::from(125));
        points_col_p_c.insert(Mfp::from(73), Mfp::from(59));
        points_col_p_c.insert(Mfp::from(62), Mfp::from(1));
        points_col_p_c.insert(Mfp::from(132), Mfp::from(1));
        points_col_p_c.insert(Mfp::from(65), Mfp::from(42));
        points_col_p_c.insert(Mfp::from(80), Mfp::from(59));

        let points_val_p_c = get_matrix_point_val(
            &self.commitm.matrices.c,
            &self.commitm.set_h,
            &self.commitm.set_k,
            &points_row_p_c,
            &points_col_p_c,
        );

        let a_row_px = sigma_yi_li(&points_row_p_a, &self.commitm.set_k);
        println!("a_row_px: ");
        dsp_poly!(a_row_px);
        let a_col_px = sigma_yi_li(&points_col_p_a, &self.commitm.set_k);
        println!("a_col_px: ");
        dsp_poly!(a_col_px);
        let a_val_px = sigma_yi_li(&points_val_p_a, &self.commitm.set_k);
        println!("a_val_px: ");
        dsp_poly!(a_val_px);

        let b_row_px = sigma_yi_li(&points_row_p_b, &self.commitm.set_k);
        println!("b_row_px: ");
        dsp_poly!(b_row_px);
        let b_col_px = sigma_yi_li(&points_col_p_b, &self.commitm.set_k);
        println!("b_col_px: ");
        dsp_poly!(b_col_px);
        let b_val_px = sigma_yi_li(&points_val_p_b, &self.commitm.set_k);
        println!("b_val_px: ");
        dsp_poly!(b_val_px);

        let c_row_px = sigma_yi_li(&points_row_p_c, &self.commitm.set_k);
        println!("c_row_px: ");
        dsp_poly!(c_row_px);
        let c_col_px = sigma_yi_li(&points_col_p_c, &self.commitm.set_k);
        println!("c_col_px: ");
        dsp_poly!(c_col_px);
        let c_val_px = sigma_yi_li(&points_val_p_c, &self.commitm.set_k);
        println!("c_val_px: ");
        dsp_poly!(c_val_px);

        let polys_pxs = vec![
            a_row_px, a_col_px, a_val_px, 
            b_row_px, b_col_px, b_val_px, 
            c_row_px, c_col_px, c_val_px,
        ];

        let points_vector = vec![
            points_val_p_a,
            points_row_p_a,
            points_col_p_a,
            points_val_p_b,
            points_row_p_b,
            points_col_p_b,
            points_val_p_c,
            points_row_p_c,
            points_col_p_c,
        ];

        self.commitm.points_px = points_vector;
        self.commitm.polys_px = polys_pxs;

        self.clone()
    }

    pub fn build(&self) -> Commitment {
        Commitment {
            ..self.commitm.clone()
        }
    }
}