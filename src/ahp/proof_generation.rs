// Copyright 2024 Fidesinnova, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::iter::repeat_with;

use anyhow::Result;
use ark_ff::Field;
use nalgebra::DMatrix;
use rand::Rng;
use rustnomial::Evaluable;
use rustnomial::FreeSizePolynomial;
use rustnomial::SizedPolynomial;
use serde::Deserialize;
use serde::Serialize;

use crate::ahp::commitment_generation::CommitmentBuilder;
use crate::dsp_mat;
use crate::dsp_poly;
use crate::dsp_vec;
use crate::json_file::write_set;
use crate::json_file::write_term;
use crate::json_file::ClassData;
use crate::kzg;
use crate::math::*;
use crate::matrices::matrix_size;
use crate::matrices::matrix_t_zeros;
use crate::matrices::Matrices;
use crate::matrices::ProgramParamsJson;
use crate::parser::Gate;
use crate::parser::GateType;
use crate::parser::RegData;
use crate::println_dbg;
use crate::to_bint;
use crate::utils::*;

use super::commitment_generation::CommitmentJson;

#[derive(Debug, Clone, Copy)]
pub enum Polys {
    WHat,
    ZHatA,
    ZHatB,
    ZHatC,
    H0,
    Sx,
    G1x,
    H1x,
    G2x,
    H2x,
    G3x,
    H3x,
}

// Assuming AHPData is defined as follows
#[derive(Serialize, Deserialize, Debug)]
pub enum AHPData {
    Commit(u64),
    Value(u64),
    Sigma(u64),
    Polynomial(Vec<u64>),
    Array(Vec<u64>),
}
pub struct ProofGeneration;
impl ProofGeneration {
    pub fn new() -> Self {
        Self
    }

    fn generate_z_interpolations(
        matrix_oz: [Vec<Mfp>; 3],
        random_b: u64,
        set_h: &Vec<Mfp>,
    ) -> (Poly, Poly, Poly) {
        let mut points_za = get_points_set(&matrix_oz[0], &set_h);
        let mut points_zb = get_points_set(&matrix_oz[1], &set_h);
        let mut points_zc = get_points_set(&matrix_oz[2], &set_h);

        // TODO: Random values were taken from WIKI. After the test is completed, these inserts should be deleted or commented out.
        // Wiki link: [https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/3-proof-generation-phase#id-3-5-2-ahp-proof]
        // Uncomment and adjust the line below to push random points
        // push_random_points(&mut points_za, random_b, &vec_to_set(set_h));
        // push_random_points(&mut points_zb, random_b, &vec_to_set(set_h));
        // push_random_points(&mut points_zc, random_b, &vec_to_set(set_h));

        points_za.push((Mfp::from(150), Mfp::from(5)));
        points_za.push((Mfp::from(80), Mfp::from(47)));
        // Random inertation for zb:
        points_zb.push((Mfp::from(150), Mfp::from(15)));
        points_zb.push((Mfp::from(80), Mfp::from(170)));
        // Random inertation for zc:
        points_zc.push((Mfp::from(150), Mfp::from(1)));
        points_zc.push((Mfp::from(80), Mfp::from(100)));

        let poly_z_hat_a = interpolate(&points_za);
        let poly_z_hat_b = interpolate(&points_zb);
        let poly_z_hat_c = interpolate(&points_zc);

        (poly_z_hat_a, poly_z_hat_b, poly_z_hat_c)
    }

    // Helper function to compute interpolations for w(h)
    fn compute_x_w_vanishing_interpolation(
        random_b: u64,
        set_h: &Vec<Mfp>,
        x_vec: &Vec<Mfp>,
        cz_mat_vec: &Vec<Mfp>,
        numebr_t_zero: usize,
    ) -> (Poly, Poly, Poly) {
        // Split set_h into two subsets based on index t
        let set_h_1 = &set_h[0..numebr_t_zero].to_vec(); // H[>∣x∣]
        let set_h_2 = &set_h[numebr_t_zero..].to_vec(); // H[<=∣x∣]

        // Interpolate polynomial for x^(h) over the subset H[>∣x∣]
        let points = get_points_set(&x_vec[0..numebr_t_zero], set_h_1);
        let poly_x_hat = interpolate(&points);
        // Interpolate polynomial w(h) over the subset H[<=∣x∣]
        // FIXME:
        let points = get_points_set(&cz_mat_vec[numebr_t_zero..], set_h_2);
        let w_hat = interpolate(&points);

        // Compute the vanishing polynomial for the subset H[<=∣x∣]
        let van_poly_vh1 = vanishing_poly(set_h_1);
        println_dbg!("van_poly_vh1: ");
        dsp_poly!(van_poly_vh1);

        let mut points_w = vec![];
        for i in set_h_2 {
            // Compute the adjusted polynomial wˉ(h) for each element in the subset

            let w_bar_h = (w_hat.eval(*i) - poly_x_hat.eval(*i))
                * exp_mod(to_bint!(van_poly_vh1.eval(*i)), P - 2);
            points_w.push((*i, w_bar_h));
        }

        // TODO:
        // Uncomment this line to insert random points for wˉ(h) from the set
        // push_random_points(&mut points_w, random_b, &vec_to_set(&set_h));
        // From wiki: [https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/3-proof-generation-phase#id-3-5-2-ahp-proof]
        points_w.push((Mfp::from(150), Mfp::from(42)));
        points_w.push((Mfp::from(80), Mfp::from(180)));

        // Interpolate polynomial for wˉ(h) based on the points_w
        let poly_w_hat = interpolate(&points_w);

        (poly_x_hat, poly_w_hat, van_poly_vh1)
    }

    fn calculate_r_polynomials_with_alpha(
        points_px: &Vec<HashMap<Mfp, Mfp>>,
        alpha: Mfp,
        set_h: &Vec<Mfp>,
    ) -> (Poly, Poly, Poly) {
        // ∑ r(alpha_2=10, k) * A^(k,x)
        let r_a_kx = sigma_rk_mk(
            set_h,
            alpha,
            &points_px[0],
            &points_px[1],
            &points_px[2],
            &EvalOrder::KX,
        );

        println_dbg!("Poly ∑ r(alpha_2=10, k) * A^(k,x): A_h");
        dsp_poly!(r_a_kx);

        // ∑ r(alpha_2=10, k) * B^(k,x)
        let r_b_kx = sigma_rk_mk(
            set_h,
            alpha,
            &points_px[3],
            &points_px[4],
            &points_px[5],
            &EvalOrder::KX,
        );
        println_dbg!("Poly ∑ r(alpha_2=10, k) * B^(k,x): ");
        dsp_poly!(r_b_kx);

        // ∑ r(alpha_2=10, k) * C^(k,x)
        let r_c_kx = sigma_rk_mk(
            set_h,
            alpha,
            &points_px[6],
            &points_px[7],
            &points_px[8],
            &EvalOrder::KX,
        );
        println_dbg!("Poly ∑ r(alpha_2=10, k) * C^(k,x): ");
        dsp_poly!(r_c_kx);

        (r_a_kx, r_b_kx, r_c_kx)
    }

    fn calculate_r_polynomials_with_beta(
        points_px: &Vec<HashMap<Mfp, Mfp>>,
        beta_1: Mfp,
        set_h: &Vec<Mfp>,
    ) -> (Poly, Poly, Poly) {
        // ∑ r(alpha_2=10, k) * A^(x,k)
        let r_a_xk = m_k(
            &beta_1,
            &points_px[0],
            &points_px[1],
            &points_px[2],
            set_h.len(),
            &EvalOrder::XK,
        );
        println_dbg!("Poly ∑ r(alpha_2=10, k) * A^(x,k): ");
        dsp_poly!(r_a_xk);

        // ∑ r(alpha_2=10, k) * B^(x,k)
        let r_b_xk = m_k(
            &beta_1,
            &points_px[3],
            &points_px[4],
            &points_px[5],
            set_h.len(),
            &EvalOrder::XK,
        );
        println_dbg!("Poly ∑ r(alpha_2=10, k) * B^(x,k): ");
        dsp_poly!(r_b_xk);

        // ∑ r(alpha_2=10, k) * C^(x,k)
        let r_c_xk = m_k(
            &beta_1,
            &points_px[6],
            &points_px[7],
            &points_px[8],
            set_h.len(),
            &EvalOrder::XK,
        );
        println_dbg!("Poly ∑ r(alpha_2=10, k) * C^(x,k): ");
        dsp_poly!(r_c_xk);

        (r_a_xk, r_b_xk, r_c_xk)
    }

    fn get_points_px_vec(
        set_h: &Vec<Mfp>,
        set_k: &Vec<Mfp>,
        matrices: Vec<&DMatrix<Mfp>>,
    ) -> Vec<HashMap<Mfp, Mfp>> {
        // Matrix A:
        let mut points_row_p_a = get_matrix_point_row(&matrices[0], &set_h, &set_k);
        // rowA' = (48, 1), (73, 135), (62, 125), (132, 59), (65, 42), (80, 1)
        // points_row_p_a.insert(Mfp::from(48), Mfp::from(1));
        // points_row_p_a.insert(Mfp::from(73), Mfp::from(135));
        // points_row_p_a.insert(Mfp::from(62), Mfp::from(125));
        // points_row_p_a.insert(Mfp::from(132), Mfp::from(59));
        // points_row_p_a.insert(Mfp::from(65), Mfp::from(42));
        // points_row_p_a.insert(Mfp::from(80), Mfp::from(1));

        let mut points_col_p_a = get_matrix_point_col(&matrices[0], &set_h, &set_k);
        // colA' = (48, 42), (73, 1), (62, 135), (132, 125), (65, 59), (80, 42)
        // points_col_p_a.insert(Mfp::from(48), Mfp::from(42));
        // points_col_p_a.insert(Mfp::from(73), Mfp::from(1));
        // points_col_p_a.insert(Mfp::from(62), Mfp::from(135));
        // points_col_p_a.insert(Mfp::from(132), Mfp::from(125));
        // points_col_p_a.insert(Mfp::from(65), Mfp::from(59));
        // points_col_p_a.insert(Mfp::from(80), Mfp::from(42));

        let points_val_p_a = get_matrix_point_val(
            &matrices[0],
            &set_h,
            &set_k,
            &points_row_p_a,
            &points_col_p_a,
        );

        // Matrix B:
        let mut points_row_p_b = get_matrix_point_row(&matrices[1], &set_h, &set_k);
        // rowB' = (73, 59), (62, 1), (132, 42), (65, 135), (80, 59)
        // points_row_p_b.insert(Mfp::from(73), Mfp::from(59));
        // points_row_p_b.insert(Mfp::from(62), Mfp::from(1));
        // points_row_p_b.insert(Mfp::from(132), Mfp::from(42));
        // points_row_p_b.insert(Mfp::from(65), Mfp::from(135));
        // points_row_p_b.insert(Mfp::from(80), Mfp::from(59));

        let mut points_col_p_b = get_matrix_point_col(&matrices[1], &set_h, &set_k);
        // colB' = (73, 59), (62, 42), (132, 125), (65, 1), (80, 135)
        // points_col_p_b.insert(Mfp::from(73), Mfp::from(59));
        // points_col_p_b.insert(Mfp::from(62), Mfp::from(42));
        // points_col_p_b.insert(Mfp::from(132), Mfp::from(125));
        // points_col_p_b.insert(Mfp::from(65), Mfp::from(1));
        // points_col_p_b.insert(Mfp::from(80), Mfp::from(135));

        let points_val_p_b = get_matrix_point_val(
            &matrices[1],
            &set_h,
            &set_k,
            &points_row_p_b,
            &points_col_p_b,
        );

        // Matrix C
        let mut points_row_p_c = get_matrix_point_row(&matrices[2], &set_h, &set_k);
        // rowC' = (48, 1), (73, 59), (62, 125), (132, 1), (65, 135), (80, 42)
        // points_row_p_c.insert(Mfp::from(48), Mfp::from(1));
        // points_row_p_c.insert(Mfp::from(73), Mfp::from(59));
        // points_row_p_c.insert(Mfp::from(62), Mfp::from(125));
        // points_row_p_c.insert(Mfp::from(132), Mfp::from(1));
        // points_row_p_c.insert(Mfp::from(65), Mfp::from(135));
        // points_row_p_c.insert(Mfp::from(80), Mfp::from(42));

        let mut points_col_p_c = get_matrix_point_col(&matrices[2], &set_h, &set_k);
        // colC' = (48, 125), (73, 59), (62, 1), (132, 1), (65, 42), (80, 59)
        // points_col_p_c.insert(Mfp::from(48), Mfp::from(125));
        // points_col_p_c.insert(Mfp::from(73), Mfp::from(59));
        // points_col_p_c.insert(Mfp::from(62), Mfp::from(1));
        // points_col_p_c.insert(Mfp::from(132), Mfp::from(1));
        // points_col_p_c.insert(Mfp::from(65), Mfp::from(42));
        // points_col_p_c.insert(Mfp::from(80), Mfp::from(59));

        let points_val_p_c = get_matrix_point_val(
            &matrices[2],
            &set_h,
            &set_k,
            &points_row_p_c,
            &points_col_p_c,
        );

        vec![
            points_val_p_a,
            points_row_p_a,
            points_col_p_a,
            points_val_p_b,
            points_row_p_b,
            points_col_p_b,
            points_val_p_c,
            points_row_p_c,
            points_col_p_c,
        ]
    }

    /// Retrieves matrices A, B, and C based on the provided matrices JSON and class data.
    ///
    /// # Parameters
    /// - `matrices`: A reference to a `MatricesJson` object containing matrix data.
    /// - `class_data`: A reference to a `ClassData` object used to determine the size of the matrices.
    ///
    /// # Returns
    /// A tuple containing three dense matrices: (A, B, C).
    fn get_matrices(
        matrices: &ProgramParamsJson,
        class_data: &ClassData,
    ) -> (DMatrix<Mfp>, DMatrix<Mfp>, DMatrix<Mfp>) {
        let a = matrices.get_matrix_a(matrix_size(&class_data), matrix_t_zeros(&class_data));
        let b = matrices.get_matrix_b(matrix_size(&class_data));
        let c = Matrices::generate_matrix_c(matrix_size(&class_data), matrix_t_zeros(&class_data));

        (a, b, c)
    }

    pub fn generate_mat_z(gates: Vec<Gate>, class_data: &ClassData) -> DMatrix<Mfp> {
        let size = matrix_size(&class_data);
        let mut regs_data: HashMap<u8, RegData> = HashMap::new();
        let mut _index = 0;
        let mut val_counter: usize = 0;
        for (_, gate) in gates.iter().enumerate() {
            if gate.gate_type == GateType::Ld {
                let right_val = gate.val_right.map_or(Mfp::ZERO, Mfp::from);
                match regs_data.contains_key(&gate.reg_right) {
                    true => panic!("The register has been loaded again!"),
                    false => regs_data.insert(gate.reg_right, RegData::new(right_val)),
                };
                continue;
            }
            CommitmentBuilder::add_val(&mut regs_data, gate, gate.gate_type, &mut val_counter);
        }
        let mut matrix_z = DMatrix::<Mfp>::zeros(size, 1);
        Self::gen_z_mat(&mut matrix_z, &regs_data);

        println_dbg!("Mat Z Proof:");
        dsp_mat!(matrix_z);

        matrix_z
    }

    fn gen_z_mat(z_vec: &mut DMatrix<Mfp>, regs_data: &HashMap<u8, RegData>) {
        z_vec[(0, 0)] = Mfp::ONE;
        let mut z_vec_counter: usize = 1;

        let mut witnesses: Vec<(usize, Mfp)> = vec![];
        let mut final_val = vec![];
        for reg in 0..32 {
            if regs_data.contains_key(&reg) {
                let data = regs_data.get(&reg).unwrap();
                // println_dbg!("data here ==> {:?}", data);
                z_vec[(z_vec_counter, 0)] = data.init_val;
                z_vec_counter += 1;
                let mut witness = data.witness.clone();
                if witness.is_empty() {
                    continue;
                }
                let last_val = witness.pop().unwrap();
                witnesses.extend(witness.iter());
                final_val.push(last_val.1);
            }
        }

        witnesses.sort();

        for w in witnesses {
            z_vec[(z_vec_counter, 0)] = w.1;
            // println!("w: {}", w);
            z_vec_counter += 1;
        }

        for f in final_val.iter().rev() {
            z_vec[(z_vec_counter, 0)] = *f;
            z_vec_counter += 1;
        }
    }

    pub fn generate_proof(
        &self,
        commitment_key: &Vec<Mfp>,
        class_data: ClassData,
        matrices: ProgramParamsJson,
        commitment_json: CommitmentJson,
        gates: Vec<Gate>,
    ) -> Box<[AHPData]> {
        // Generate sets
        let set_h = generate_set(class_data.n);
        let set_k = generate_set(class_data.m);

        let numebr_t_zero = matrix_t_zeros(&class_data);
        let (mat_a, mat_b, mat_c) = Self::get_matrices(&matrices, &class_data);
        let mat_z = Self::generate_mat_z(gates, &class_data);
        let points_px = Self::get_points_px_vec(&set_h, &set_k, vec![&mat_a, &mat_b, &mat_c]);
        let x_vec = &mat_to_vec(&mat_z)[..numebr_t_zero];

        // TODO: Set 'random_b' to a random value in the range 1 to 50
        let random_b = 2;

        // Generate and interpolate points for matrices az, bz, cz
        let (poly_z_hat_a, poly_z_hat_b, poly_z_hat_c) = Self::generate_z_interpolations(
            [
                mat_to_vec(&(&mat_a * &mat_z)),
                mat_to_vec(&(&mat_b * &mat_z)),
                mat_to_vec(&(&mat_c * &mat_z)),
            ],
            random_b,
            &set_h,
        );

        let (poly_x_hat, poly_w_hat, van_poly_vh1) = Self::compute_x_w_vanishing_interpolation(
            random_b,
            &set_h,
            &x_vec.to_vec(),
            &mat_to_vec(&(&mat_c * &mat_z)),
            numebr_t_zero,
        );
        println_dbg!("w_hat:"); // Output the interpolated polynomial for wˉ(h)
        dsp_poly!(poly_w_hat);

        // h_zero
        let van_poly_vhx = vanishing_poly(&set_h);

        println_dbg!("van_poly_vhx: ");
        dsp_poly!(van_poly_vhx);

        let poly_ab_c = &poly_z_hat_a * &poly_z_hat_b - &poly_z_hat_c;
        println_dbg!("{:?} ,,, {:?}", poly_ab_c, van_poly_vhx);
        let poly_h_0 = div_mod(&poly_ab_c, &van_poly_vhx).0;
        println_dbg!("h0(x):");
        dsp_poly!(poly_h_0);

        // Generate a random polynomial
        let poly_sx = Self::generate_random_polynomial(2 * set_h.len() + 2 - 1, (0, P));
        println_dbg!("poly_sx");
        dsp_poly!(poly_sx);

        // Compute sigma by evaluating the polynomial at points in set_h
        let sigma_1 = set_h
            .iter()
            .fold(Mfp::ZERO, |acc, &v| acc + poly_sx.eval(v));
        println_dbg!("sigma_1 :	{}", sigma_1);

        // TODO:
        // let alpha = Mfp::from(sha2_hash(&(poly_sx.eval(Mfp::from(0))).to_string()));
        // let eta_a = Mfp::from(sha2_hash(&(poly_sx.eval(Mfp::from(1))).to_string()));
        // let eta_b = Mfp::from(sha2_hash(&(poly_sx.eval(Mfp::from(2))).to_string()));
        // let eta_c = Mfp::from(sha2_hash(&(poly_sx.eval(Mfp::from(3))).to_string()));

        // From wiki: [https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/3-proof-generation-phase#id-3-5-2-ahp-proof]
        //             Step 6
        let alpha = Mfp::from(10);
        let eta_a = Mfp::from(2);
        let eta_b = Mfp::from(30);
        let eta_c = Mfp::from(100);

        // Compute polynomial for ∑ ηz(x)
        let sigma_eta_z_x = Poly::new(vec![eta_a]) * &poly_z_hat_a
            + Poly::new(vec![eta_b]) * &poly_z_hat_b
            + Poly::new(vec![eta_c]) * &poly_z_hat_c;

        // Compute polynomial for r(α,x) ∑ ηM(z^M(x))
        let poly_r = func_u(Some(alpha), None, set_h.len());
        println_dbg!("r:");
        dsp_poly!(poly_r);

        println_dbg!("r(alpha_2 , x) ∑_m [η_M z^_M(x)]:");
        dsp_poly!((&poly_r * &sigma_eta_z_x));

        // r(α,x) * ∑_m [η_M ​z^M​(x)]
        let sum_1 = &poly_r * sigma_eta_z_x;

        // Compute polynomial for Z^(x)
        let poly_z_hat_x = &poly_w_hat * &van_poly_vh1 + poly_x_hat;
        println_dbg!("z_hat: ");
        dsp_poly!(poly_z_hat_x);

        let (r_a_kx, r_b_kx, r_c_kx) =
            Self::calculate_r_polynomials_with_alpha(&points_px, alpha, &set_h);

        // ∑_m [η_M r_M(α,x)] * z^(x)
        let sum_2 = Poly::new(vec![eta_a]) * &r_a_kx
            + Poly::new(vec![eta_b]) * &r_b_kx
            + Poly::new(vec![eta_c]) * &r_c_kx;
        let sum_2 = sum_2 * &poly_z_hat_x;

        // Sum Check Protocol Formula:
        // s(x) + r(α,x) * ∑_m [η_M ​z^M​(x)] - ∑_m [η_M r_M(α,x)] * z^(x)
        let poly_scp = poly_sx.clone() + sum_1.clone() - &sum_2;

        println_dbg!("scp: ");
        dsp_poly!(poly_scp);

        let div_res = div_mod(&poly_scp, &van_poly_vhx);
        let h_1x = div_res.0;
        println_dbg!("Poly h_1x: ");
        dsp_poly!(h_1x);

        let g_1x = div_mod(&div_res.1, &Poly::new(vec![Mfp::ONE, Mfp::ZERO])).0;
        println_dbg!("Poly g_1x:");
        dsp_poly!(g_1x);

        // TODO: Random F - H
        // let beta_1 = gen_rand_not_in_set(&vec_to_set(&set_h));
        // let beta_1 = Mfp::from(sha2_hash(&poly_sx.eval(Mfp::from(9)).to_string()));
        // let beta_2 = gen_rand_not_in_set(&vec_to_set(&set_h));
        // let beta_2 = Mfp::from(sha2_hash(&poly_sx.eval(Mfp::from(10)).to_string()));
        let beta_1 = Mfp::from(22);
        let beta_2 = Mfp::from(80);
        let beta_3 = Mfp::from(5);

        let (r_a_xk, r_b_xk, r_c_xk) =
            Self::calculate_r_polynomials_with_beta(&points_px, beta_1, &set_h);

        // sigma_2
        let sigma_2 =
            eta_a * r_a_kx.eval(beta_1) + eta_b * r_b_kx.eval(beta_1) + eta_c * r_c_kx.eval(beta_1);
        println_dbg!("sigma_2: {}", sigma_2);

        // r(alpha_2=10, x) ∑_m [​η_M ​M^(x,β1​)]
        let poly_sigma_2 = Poly::new(vec![eta_a]) * r_a_xk
            + Poly::new(vec![eta_b]) * r_b_xk
            + Poly::new(vec![eta_c]) * r_c_xk;
        let poly_sigma_2 = &poly_r * poly_sigma_2;

        println_dbg!("r(alpha_2=10, x) * ∑_m [η_M M^(x, β1)]: ");
        dsp_poly!(poly_sigma_2);

        let div_res = div_mod(&poly_sigma_2, &van_poly_vhx);
        let h_2x = div_res.0;
        println_dbg!("Poly h_2x: ");
        dsp_poly!(h_2x);

        let g_2x = div_mod(&div_res.1, &Poly::new(vec![Mfp::ONE, Mfp::ZERO])).0;
        println_dbg!("Poly g_2x:");
        dsp_poly!(g_2x);

        // sigma_3
        let mut sigma_3 = Mfp::ZERO;

        let polys_px = commitment_json.get_polys_px();

        // f_3x
        let poly_f_3x = Self::generate_poly_fx(
            &mut sigma_3,
            &polys_px,
            &van_poly_vhx,
            &vec![eta_a, eta_b, eta_c],
            &vec![beta_1, beta_2],
            &set_k,
        );
        println_dbg!("poly_f_3x");
        dsp_poly!(poly_f_3x);
        println_dbg!("sigma_3: {}", sigma_3);

        let (pi_a, pi_b, pi_c) = Self::compute_polys_pi(beta_1, beta_2, &polys_px);
        let polys_pi = vec![&pi_a, &pi_b, &pi_c];

        println_dbg!("poly_pi_a");
        dsp_poly!(polys_pi[0]);
        println_dbg!("poly_pi_b");
        dsp_poly!(polys_pi[1]);
        println_dbg!("poly_pi_c");
        dsp_poly!(polys_pi[2]);

        // a(x)
        let poly_a_x = Self::generate_poly_ax(
            &polys_px,
            vec![beta_1, beta_2],
            &van_poly_vhx,
            vec![eta_a, eta_b, eta_c],
            &polys_pi,
        );
        println_dbg!("poly_a_x: {}", poly_a_x.eval(Mfp::from(5)));
        dsp_poly!(poly_a_x);

        // b(x)
        let poly_b_x = polys_pi[0] * polys_pi[1] * polys_pi[2];
        println_dbg!("poly_b_x: {}", poly_b_x.eval(Mfp::from(5)));
        dsp_poly!(poly_b_x);

        let van_poly_vkx = vanishing_poly(&set_k);
        println_dbg!("van_poly_vkx");
        dsp_poly!(van_poly_vkx);

        let sigma_3_set_k = div_mod_val(Mfp::from(sigma_3), Mfp::from(set_k.len() as u64));
        println_dbg!("sigma_3_set_k {}", sigma_3_set_k);

        let poly_f_3x = poly_f_3x - Poly::from(vec![sigma_3_set_k]);

        println_dbg!("poly_f_3x");
        dsp_poly!(poly_f_3x);

        let g_3x = div_mod(&poly_f_3x, &Poly::from(vec![Mfp::ONE, Mfp::ZERO])).0;
        println_dbg!("g_3x");
        dsp_poly!(g_3x);

        let h_3x = (poly_a_x.clone()
            - (&poly_b_x * (poly_f_3x.clone() + Poly::from(vec![sigma_3_set_k]))))
        .div_mod(&van_poly_vkx)
        .0;
        println_dbg!("h_3x");
        dsp_poly!(h_3x);

        let polys_proof = [
            poly_w_hat,
            poly_z_hat_a,
            poly_z_hat_b,
            poly_z_hat_c,
            poly_h_0,
            poly_sx,
            g_1x,
            h_1x,
            g_2x,
            h_2x,
            g_3x,
            h_3x,
        ];

        // TODO: All random (1..P)
        let eta_values = [
            Mfp::from(1),  // eta_w
            Mfp::from(4),  // eta_z_a
            Mfp::from(10), // eta_z_b
            Mfp::from(8),  // eta_z_c
            Mfp::from(32), // eta_h0
            Mfp::from(45), // eta_s
            Mfp::from(92), // eta_g1
            Mfp::from(11), // eta_h1
            Mfp::from(1),  // eta_g2
            Mfp::from(5),  // eta_h2
            Mfp::from(25), // eta_g3
            Mfp::from(63), // eta_h3
        ];

        let poly_px = eta_values
            .iter()
            .enumerate()
            .map(|(i, &eta)| Poly::from(vec![eta]) * polys_proof[i].clone())
            .fold(Poly::zero(), |acc, poly| acc + poly);

        println_dbg!("poly_px:");
        dsp_poly!(poly_px);

        // TODO:
        // hash(poly_sx(22));
        let z = Mfp::from(2);
        let val_y_p = poly_px.eval(z);
        println_dbg!("val_y_p {}", val_y_p);

        let mut poly_px_add = poly_px;
        poly_px_add.add_term(-val_y_p, 0);
        let poly_x_z = Poly::from(vec![Mfp::ONE, Mfp::from(-z)]);

        let poly_qx = div_mod(&poly_px_add, &poly_x_z).0;
        println_dbg!("poly_qx");
        dsp_poly!(poly_qx);

        let val_commit_poly_qx = kzg::commit(&poly_qx, commitment_key);
        println_dbg!("val_commit_qx: {}", val_commit_poly_qx);

        let sigma = [sigma_1, sigma_2, sigma_3];

        let commit_x = compute_all_commitment(&polys_proof, commitment_key);
        println_dbg!("commit_x: {}", dsp_vec!(commit_x));

        Self::create_proof(
            &polys_proof,
            &sigma,
            &commit_x,
            val_y_p,
            val_commit_poly_qx,
            &x_vec.to_vec(),
        )
    }

    pub fn compute_polys_pi(beta_1: Mfp, beta_2: Mfp, polys_px: &[Poly]) -> (Poly, Poly, Poly) {
        let poly_pi_a =
            (Poly::from(vec![beta_2]) - &polys_px[0]) * (Poly::from(vec![beta_1]) - &polys_px[1]);
        let poly_pi_b =
            (Poly::from(vec![beta_2]) - &polys_px[3]) * (Poly::from(vec![beta_1]) - &polys_px[4]);
        let poly_pi_c =
            (Poly::from(vec![beta_2]) - &polys_px[6]) * (Poly::from(vec![beta_1]) - &polys_px[7]);

        (poly_pi_a, poly_pi_b, poly_pi_c)
    }

    fn generate_random_polynomial(degree: usize, coefficient_range: (u64, u64)) -> Poly {
        let mut rng = rand::thread_rng();
        let coefficients: Vec<Mfp> = repeat_with(|| {
            let random_value = rng.gen_range(coefficient_range.0..=coefficient_range.1);
            Mfp::from(random_value)
        })
        .take(degree + 1) // +1 because degree is the highest power
        .collect();

        // TODO: Random numebrs from Wiki, Comment it after test
        let coefficients = [5, 0, 101, 17, 0, 1, 20, 0, 0, 3, 115]
            .iter()
            .map(|v| Mfp::from(*v))
            .collect::<Vec<Mfp>>();

        let mut rand_poly = Poly::from(coefficients);
        rand_poly.trim();
        rand_poly
    }

    fn create_proof(
        polys_proof: &[Poly],
        sigma: &[Mfp],
        commit_x: &[Mfp],
        val_y_p: Mfp,
        val_commit_poly_qx: Mfp,
        x_vec: &Vec<Mfp>,
    ) -> Box<[AHPData]> {
        let pi_ahp = [
            AHPData::Array(write_set(x_vec)),                  // COM1AHP
            AHPData::Commit(to_bint!(commit_x[0])),            // [0]: COM2AHP
            AHPData::Commit(to_bint!(commit_x[1])),            // [1]: COM3AHP
            AHPData::Commit(to_bint!(commit_x[2])),            // [2]: COM4AHP
            AHPData::Commit(to_bint!(commit_x[3])),            // [3]: COM5AHP
            AHPData::Commit(to_bint!(commit_x[4])),            // [4]: COM6AHP
            AHPData::Commit(to_bint!(commit_x[5])),            // [5]: COM7AHP
            AHPData::Commit(to_bint!(commit_x[6])),            // [6]: COM8AHP
            AHPData::Commit(to_bint!(commit_x[7])),            // [7]: COM9AHP
            AHPData::Commit(to_bint!(commit_x[8])),            // [8]: COM10AHP
            AHPData::Commit(to_bint!(commit_x[9])),            // [9]: COM11AHP
            AHPData::Commit(to_bint!(commit_x[10])),           // [10]: COM12AHP
            AHPData::Commit(to_bint!(commit_x[11])),           // [11]: COM13AHP
            AHPData::Sigma(to_bint!(sigma[0])),                // [12]: P1AHP: sigma_1
            AHPData::Sigma(to_bint!(sigma[1])),                // [21]: P10AHP: sigma_2
            AHPData::Sigma(to_bint!(sigma[2])),                // [24]: P13AHP: sigma_3
            AHPData::Value(to_bint!(val_y_p)),                 // [27]: P16AHP: y'
            AHPData::Value(to_bint!(val_commit_poly_qx)),      // [28]: P17AHP: val_com_qx
            AHPData::Polynomial(write_term(&polys_proof[0])),  // [13]: P2AHP: w^x
            AHPData::Polynomial(write_term(&polys_proof[1])),  // [14]: P3AHP: z^a
            AHPData::Polynomial(write_term(&polys_proof[2])),  // [15]: P4AHP: z^b
            AHPData::Polynomial(write_term(&polys_proof[3])),  // [16]: P5AHP: z^c
            AHPData::Polynomial(write_term(&polys_proof[4])),  // [17]: P6AHP: h_0
            AHPData::Polynomial(write_term(&polys_proof[5])),  // [18]: P7AHP: sx
            AHPData::Polynomial(write_term(&polys_proof[6])),  // [19]: P8AHP: g_1
            AHPData::Polynomial(write_term(&polys_proof[7])),  // [20]: P9AHP: h_1
            AHPData::Polynomial(write_term(&polys_proof[8])),  // [22]: P11AHP: g_2
            AHPData::Polynomial(write_term(&polys_proof[9])),  // [23]: P12AHP: h_2
            AHPData::Polynomial(write_term(&polys_proof[10])), // [25]: P14AHP: g_3
            AHPData::Polynomial(write_term(&polys_proof[11])), // [26]: P15AHP: h_3
        ];

        Box::new(pi_ahp)
    }

    fn generate_poly_fx(
        sigma_3: &mut Mfp,
        polys_px: &Vec<Poly>,
        van_poly_vhx: &Poly,
        eta: &Vec<Mfp>,
        beta: &Vec<Mfp>,
        set_k: &Vec<Mfp>,
    ) -> Poly {
        let mut points_f_3: Vec<Point> = vec![];
        for k in set_k.iter() {
            let sig_a = sigma_m(
                &van_poly_vhx,
                &eta[0],
                &beta[0],
                &beta[1],
                k,
                &[&polys_px[0], &polys_px[1], &polys_px[2]],
            );
            let sig_b = sigma_m(
                &van_poly_vhx,
                &eta[1],
                &beta[0],
                &beta[1],
                k,
                &[&polys_px[3], &polys_px[4], &polys_px[5]],
            );
            let sig_c = sigma_m(
                &van_poly_vhx,
                &eta[2],
                &beta[0],
                &beta[1],
                k,
                &[&polys_px[6], &polys_px[7], &polys_px[8]],
            );

            let sum = sig_a + sig_b + sig_c;
            *sigma_3 += sum;
            points_f_3.push((*k, sum));
        }
        interpolate(&points_f_3)
    }

    fn generate_poly_ax(
        polys_px: &Vec<Poly>,
        beta: Vec<Mfp>,
        van_poly_vhx: &Poly,
        eta: Vec<Mfp>,
        poly_pi: &Vec<&Poly>,
    ) -> Poly {
        let val_vhx_beta_1 = van_poly_vhx.eval(beta[0]);
        let val_vhx_beta_2 = van_poly_vhx.eval(beta[1]);
        let beta_mul = val_vhx_beta_2 * val_vhx_beta_1;

        let poly_sig_a = Poly::from(vec![eta[0] * beta_mul]) * &polys_px[2];
        let poly_sig_b = Poly::from(vec![eta[1] * beta_mul]) * &polys_px[5];
        let poly_sig_c = Poly::from(vec![eta[2] * beta_mul]) * &polys_px[8];

        println_dbg!("poly_sig_a");
        dsp_poly!(poly_sig_a);
        println_dbg!("poly_sig_b");
        dsp_poly!(poly_sig_b);
        println_dbg!("poly_sig_c");
        dsp_poly!(poly_sig_c);

        poly_sig_a * (poly_pi[1] * poly_pi[2])
            + poly_sig_b * (poly_pi[0] * poly_pi[2])
            + poly_sig_c * (poly_pi[0] * poly_pi[1])
    }

    /// Store in Json file
    pub fn store(&self, path: &str, proof_data: Box<[AHPData]>) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        let proof_json = ProofGenerationJson::new(proof_data);
        serde_json::to_writer(writer, &proof_json)?;
        Ok(())
    }

    /// Restore Commitment from Json file
    pub fn restore(path: &str) -> Result<ProofGenerationJson> {
        read_json_file(path)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProofGenerationJson {
    commits: Vec<u64>,
    polys: Vec<Vec<u64>>,
    sigma: Vec<u64>,
    values: Vec<u64>,
    x_vec: Vec<u64>,
}

impl ProofGenerationJson {
    pub fn new(proof_data: Box<[AHPData]>) -> Self {
        let mut commits = vec![];
        let mut polys = vec![];
        let mut sigma = vec![];
        let mut values = vec![];
        let mut x_vec = vec![];

        for val in proof_data {
            match val {
                AHPData::Commit(v) => commits.push(v),
                AHPData::Value(v) => values.push(v),
                AHPData::Polynomial(v) => polys.push(v),
                AHPData::Sigma(v) => sigma.push(v),
                AHPData::Array(v) => x_vec = v,
            }
        }

        Self {
            commits,
            polys,
            sigma,
            values,
            x_vec,
        }
    }

    pub fn get_x_vec(&self) -> Vec<Mfp> {
        self.x_vec.iter().map(|v| Mfp::from(*v)).collect()
    }

    pub fn get_poly(&self, poly: usize) -> Poly {
        let poly_vec = self.polys[poly]
            .iter()
            .rev()
            .map(|&v| Mfp::from(v))
            .collect::<Vec<Mfp>>();
        let mut poly = Poly::from(poly_vec);
        poly.trim();
        poly
    }

    pub fn get_sigma(&self, num: usize) -> Mfp {
        Mfp::from(self.sigma[num - 1])
    }

    pub fn get_value(&self, val: usize) -> Mfp {
        Mfp::from(self.values[val])
    }
}



// JSON struct according to Witi (not complete)
#[derive(Serialize, Deserialize, Debug)]
struct ProofGenerationJson2 {
    #[serde(rename = "CommitmentID")]
    commitment_id: u64,
    
    #[serde(rename = "DeviceEncodedID")]
    device_encoded_id: String,
    
    #[serde(rename = "Input")]
    input: String,
    
    #[serde(rename = "Output")]
    output: String,
    
    #[serde(rename = "P1AHP")]
    p1ahp: u64,
    
    #[serde(rename = "P2AHP")]
    p2ahp: Vec<u64>,
    
    #[serde(rename = "P3AHP")]
    p3ahp: Vec<u64>,
    
    #[serde(rename = "P4AHP")]
    p4ahp: Vec<u64>,
    
    #[serde(rename = "P5AHP")]
    p5ahp: Vec<u64>,
    
    #[serde(rename = "P6AHP")]
    p6ahp: Vec<u64>,
    
    #[serde(rename = "P7AHP")]
    p7ahp: Vec<u64>,
    
    #[serde(rename = "P8AHP")]
    p8ahp: Vec<u64>,
    
    #[serde(rename = "P9AHP")]
    p9ahp: Vec<u64>,
    
    #[serde(rename = "P10AHP")]
    p10ahp: u64,
    
    #[serde(rename = "P11AHP")]
    p11ahp: Vec<u64>,
    
    #[serde(rename = "P12AHP")]
    p12ahp: Vec<u64>,
    
    #[serde(rename = "P13AHP")]
    p13ahp: u64,
    
    #[serde(rename = "P14AHP")]
    p14ahp: Vec<u64>,
    
    #[serde(rename = "P15AHP")]
    p15ahp: Vec<u64>,
    
    #[serde(rename = "P16AHP")]
    p16ahp: u64,

    #[serde(rename = "Protocol")]
    protocol: String,
}

impl ProofGenerationJson2 {
    pub fn new(proof_data: Box<[AHPData]>) -> Self {
        let mut commits = vec![];
        let mut polys = vec![];
        let mut sigma = vec![];
        let mut values = vec![];
        let mut x_vec = vec![];

        for val in proof_data {
            match val {
                AHPData::Commit(v) => commits.push(v),
                AHPData::Value(v) => values.push(v),
                AHPData::Polynomial(v) => polys.push(v),
                AHPData::Sigma(v) => sigma.push(v),
                AHPData::Array(v) => x_vec = v,
            }
        }

        let commitment_id = sha2_hash("concat_vals(DeviceConfig.json)");

        Self {
            commitment_id,
            device_encoded_id: "Base64<MAC>".to_owned(),
            input: "None".to_owned(),
            output: "None".to_owned(),
            p1ahp: sigma[0],
            p2ahp: polys[0].clone(),
            p3ahp: polys[1].clone(),
            p4ahp: polys[2].clone(),
            p5ahp: polys[3].clone(),
            p6ahp: polys[4].clone(),
            p7ahp: polys[5].clone(),
            p8ahp: polys[6].clone(),
            p9ahp: polys[7].clone(),
            p10ahp: sigma[1],
            p11ahp: polys[8].clone(),
            p12ahp: polys[9].clone(),
            p13ahp: sigma[2],
            p14ahp: polys[10].clone(),
            p15ahp: polys[11].clone(),
            p16ahp: values[0],
            protocol: "None".to_owned(),
        }
    }

    pub fn get_x_vec(&self) -> Vec<Mfp> {
        // self.x_vec.iter().map(|v| Mfp::from(*v)).collect()
        todo!()
    }

    pub fn get_poly(&self, poly: usize) -> Poly {
        // let poly_vec = self.polys[poly]
        //     .iter()
        //     .rev()
        //     .map(|&v| Mfp::from(v))
        //     .collect::<Vec<Mfp>>();
        // let mut poly = Poly::from(poly_vec);
        // poly.trim();
        // poly
        todo!()
    }

    pub fn get_sigma(&self, num: usize) -> Mfp {
        Mfp::from(match num {
            1 => self.p1ahp,
            2 => self.p10ahp,
            3 => self.p13ahp,
            _ => panic!("Invalid sigma number")
        })
    }

    pub fn get_value(&self, val: usize) -> Mfp {
        // Mfp::from(self.values[val])
        todo!()
    }
}