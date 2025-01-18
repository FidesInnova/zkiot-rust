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
use rand::thread_rng;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;

use crate::field::fmath;
use crate::field::fmath::inverse_mul;
use crate::fpoly;
use crate::json_file::write_set;
use crate::json_file::write_term;
use crate::json_file::ClassDataJson;
use crate::json_file::DeviceInfo;
use crate::json_file::ProgramParamsJson;
use crate::kzg;
use crate::math::*;
use crate::matrices::matrix_fmath;
use crate::poly_add_many;
use crate::poly_mul_many;
use crate::polynomial::poly_fmath;
use crate::polynomial::FPoly;
use crate::println_dbg;
use crate::utils::*;

use super::commitment_generation::CommitmentJson;

/// Enum representing different polynomial types used in the computation
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

    // /// Generates a vector Z
    // pub fn generate_z_vec(class_data: &ClassDataJson, z_vec_in: Vec<u64>, p: u64) -> <u64> {
    //     let size = class_data.get_matrix_size();
    //     let mut z_vec: DVector<u64> = DVector::zeros(size);

    //     assert_eq!(z_vec_in.len() as u64, class_data.n_g + class_data.n_i + 1);

    //     for (i, z) in vals.iter().enumerate() {
    //         z_vec[(i, 0)] = *z;
    //     }

    //     println_dbg!("Mat Z Proof: {:?}", z_vec);

    //     z_vec
    // }

    /// Generates interpolated polynomials from the given matrix and random values
    fn generate_oz_interpolations(
        matrix_oz: [Vec<u64>; 3],
        random_b: u64,
        set_h: &Vec<u64>,
        p: u64
    ) -> (FPoly, FPoly, FPoly) {
        let mut points_za = get_points_set(&matrix_oz[0], &set_h);
        let mut points_zb = get_points_set(&matrix_oz[1], &set_h);
        let mut points_zc = get_points_set(&matrix_oz[2], &set_h);

        // TODO: Random values were taken from WIKI. After the test is completed, these inserts should be deleted or commented out.
        // Wiki link: [https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/3-proof-generation-phase#id-3-5-2-ahp-proof]
        // Uncomment and adjust the line below to push random points
        push_random_points(&mut points_za, random_b, &vec_to_set(set_h), p);
        push_random_points(&mut points_zb, random_b, &vec_to_set(set_h), p);
        push_random_points(&mut points_zc, random_b, &vec_to_set(set_h), p);

        println_dbg!("points_za: {:?}", points_za);
        println_dbg!("points_zb: {:?}", points_zb);
        println_dbg!("points_zc: {:?}", points_zc);

        let poly_z_hat_a = interpolate(&points_za, p);
        let poly_z_hat_b = interpolate(&points_zb, p);
        let poly_z_hat_c = interpolate(&points_zc, p);

        (poly_z_hat_a, poly_z_hat_b, poly_z_hat_c)
    }

    /// Helper function to compute interpolations for w(h)
    fn compute_x_w_vanishing_interpolation(
        random_b: u64,
        set_h: &Vec<u64>,
        z_vec: &Vec<u64>,
        numebr_t_zero: usize,
        p: u64
    ) -> (FPoly, FPoly, FPoly) {
        // Split set_h into two subsets based on index t
        let set_h_1 = &set_h[0..numebr_t_zero].to_vec(); // H[>∣x∣]
        let set_h_2 = &set_h[numebr_t_zero..].to_vec(); // H[<=∣x∣]

        // Interpolate polynomial for x^(h) over the subset H[>∣x∣]
        let points = get_points_set(&z_vec[..numebr_t_zero], set_h_1);
        let poly_x_hat = interpolate(&points, p);

        // Interpolate polynomial w(h) over the subset H[<=∣x∣]
        let points = get_points_set(&z_vec[numebr_t_zero..], set_h_2);
        println_dbg!("points w_hat {:?}", points);
        let w_hat = interpolate(&points, p);

        // Compute the vanishing polynomial for the subset H[<=∣x∣]
        let van_poly_vh1 = vanishing_poly(set_h_1, p);
        println_dbg!("van_poly_vh1: {}", van_poly_vh1);

        let mut points_w = vec![];
        for i in set_h_2 {
            // Compute the adjusted polynomial wˉ(h) for each element in the subset

            let tmp_sub = fmath::sub(w_hat.evaluate(*i, p), poly_x_hat.evaluate(*i, p), p);
            let w_bar_h = fmath::mul(tmp_sub, inverse_mul(van_poly_vh1.evaluate(*i, p), p), p);

            points_w.push((*i, w_bar_h));
        }

        // TODO:
        // Uncomment this line to insert random points for wˉ(h) from the set
        push_random_points(&mut points_w, random_b, &vec_to_set(&set_h), p);
        // From wiki: [https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/3-proof-generation-phase#id-3-5-2-ahp-proof]

        println_dbg!("points_w: {:?}\nlen: {}", points_w, points_w.len());

        // Interpolate polynomial for wˉ(h) based on the points_w
        let poly_w_hat = interpolate(&points_w, p);

        println_dbg!("poly_x_hat: {}", poly_x_hat);

        (poly_x_hat, poly_w_hat, van_poly_vh1)
    }

    /// Calculates r polynomials using alpha for given points
    fn calculate_r_polynomials_with_alpha(
        points_px: &Vec<HashMap<u64, u64>>,
        alpha: u64,
        set_h: &Vec<u64>,
        g: u64,
        p: u64
    ) -> (FPoly, FPoly, FPoly) {
        // ∑ r(alpha_2, k) * A^(k,x)
        let r_a_kx = sigma_rk_mk(
            set_h,
            alpha,
            // points A
            &points_px[0],
            &points_px[1],
            &points_px[2],
            &EvalOrder::KX,
            p
        );

        println_dbg!("Poly ∑ r(alpha_2, k) * A^(k,x): ");
        println_dbg!("{}", r_a_kx);

        // ∑ r(alpha_2, k) * B^(k,x)
        let r_b_kx = sigma_rk_mk(
            set_h,
            alpha,
            // points B
            &points_px[3],
            &points_px[4],
            &points_px[5],
            &EvalOrder::KX,
            p
        );
        println_dbg!("Poly ∑ r(alpha_2, k) * B^(k,x): ");
        println_dbg!("{}", r_b_kx);

        // ∑ r(alpha_2, k) * C^(k,x)
        let r_c_kx = sigma_rk_mk(
            set_h,
            alpha,
            // points C
            &points_px[6],
            &points_px[7],
            &points_px[8],
            &EvalOrder::KX,
            p
        );
        println_dbg!("Poly ∑ r(alpha_2, k) * C^(k,x): ");
        println_dbg!("{}", r_c_kx);

        (r_a_kx, r_b_kx, r_c_kx)
    }

    /// Calculates r polynomials using beta for given points
    fn calculate_r_polynomials_with_beta(
        points_px: &Vec<HashMap<u64, u64>>,
        beta_1: u64,
        set_h: &Vec<u64>,
        p: u64
    ) -> (FPoly, FPoly, FPoly) {
        // ∑ r(alpha_2, k) * A^(x,k)
        let r_a_xk = m_k(
            &beta_1,
            &points_px[0],
            &points_px[1],
            &points_px[2],
            set_h.len(),
            &EvalOrder::XK,
            p
        );
        println_dbg!("Poly ∑ r(alpha_2, k) * A^(x,k): ");
        println_dbg!("{}", r_a_xk);

        // ∑ r(alpha_2, k) * B^(x,k)
        let r_b_xk = m_k(
            &beta_1,
            &points_px[3],
            &points_px[4],
            &points_px[5],
            set_h.len(),
            &EvalOrder::XK,
            p
        );
        println_dbg!("Poly ∑ r(alpha_2, k) * B^(x,k): ");
        println_dbg!("{}", r_b_xk);

        // ∑ r(alpha_2, k) * C^(x,k)
        let r_c_xk = m_k(
            &beta_1,
            &points_px[6],
            &points_px[7],
            &points_px[8],
            set_h.len(),
            &EvalOrder::XK,
            p
        );
        println_dbg!("Poly ∑ r(alpha_2, k) * C^(x,k): ");
        println_dbg!("{}", r_c_xk);

        (r_a_xk, r_b_xk, r_c_xk)
    }

    /// Generates proof values to be used for creating a JSON file later
    pub fn generate_proof(
        &self,
        commitment_key: &Vec<u64>,
        class_data: ClassDataJson,
        program_params: ProgramParamsJson,
        commitment_json: CommitmentJson,
        z_vec: Vec<u64>,
        p: u64
    ) -> Box<[AHPData]> {
        // Generate sets
        let set_h = generate_set(class_data.n, class_data, p);
        let set_k = generate_set(class_data.m, class_data, p);

        let numebr_t_zero = class_data.get_matrix_t_zeros();
        let matrices = program_params.get_matrices(&class_data, p);
        let (mat_a, mat_b, mat_c) = matrices.clone();

        println_dbg!("P Mat A:");
        println_dbg!("{}", mat_a);
        println_dbg!("P Mat B:");
        println_dbg!("{}", mat_b);
        println_dbg!("P Mat C:");
        println_dbg!("{}", mat_c);

        println_dbg!("{:?}", z_vec);

        let points_px = program_params.get_points_px(&set_k, p);

        // TODO: Set 'random_b' to a random value
        // let b_max_rand = std::cmp::min(10, class_data.n_g);
        // let random_b = thread_rng().gen_range(1..b_max_rand);
        // println_dbg!("b = {}", random_b);
        let random_b = 2;

        // Generate and interpolate points for matrices az, bz, cz
        let (poly_z_hat_a, poly_z_hat_b, poly_z_hat_c) = Self::generate_oz_interpolations(
            [
                matrix_fmath::vector_mul(&mat_a, &z_vec, p),
                matrix_fmath::vector_mul(&mat_b, &z_vec, p),
                matrix_fmath::vector_mul(&mat_c, &z_vec, p),
            ],
            random_b,
            &set_h,
            p
        );

        let (poly_x_hat, poly_w_hat, van_poly_vh1) = Self::compute_x_w_vanishing_interpolation(
            random_b,
            &set_h,
            &z_vec,
            numebr_t_zero,
            p
        );
        println_dbg!("w_hat:"); // Output the interpolated polynomial for wˉ(h)
        println_dbg!("{}", poly_w_hat);

        // h_zero
        let van_poly_vhx = vanishing_poly(&set_h, p);

        println_dbg!("van_poly_vhx: ");
        println_dbg!("{}", van_poly_vhx);

        let tmp1 = poly_fmath::mul(&poly_z_hat_a, &poly_z_hat_b, p);
        let poly_ab_c = poly_fmath::sub(&tmp1, &poly_z_hat_c, p);
        
        println_dbg!("poly_ab_c");
        println_dbg!("{}", poly_ab_c);
        
        let poly_h_0 = poly_fmath::div(&poly_ab_c, &van_poly_vhx, p);

        println_dbg!("rem poly_h_0:");
        println_dbg!("{}", poly_h_0.1);

        // Ensure this division has no remainders
        assert!(
            poly_h_0.1.is_zero(),
            "Proof panic: The remainder of the division for poly_h_0 should be zero"
        );

        let poly_h_0 = poly_h_0.0;
        println_dbg!("poly_h_0");
        println_dbg!("{}", poly_h_0);

        // Generate a random polynomial
        let poly_sx = Self::generate_random_polynomial(2 * set_h.len() + 2 - 1, (0, class_data.p - 1), p);
        println_dbg!("poly_sx");
        println_dbg!("{}", poly_sx);

        // Compute sigma by evaluating the polynomial at points in set_h
        let sigma_1 = set_h
            .iter()
            .fold(0, |acc, &v| fmath::add(acc, poly_sx.evaluate(v, p), p));
        println_dbg!("sigma_1 :	{}", sigma_1);

        // TODO:
        let alpha = sha2_hash_lower_32bit(&(poly_sx.evaluate(0, p)).to_string());
        let eta_a = sha2_hash_lower_32bit(&(poly_sx.evaluate(1, p)).to_string());
        let eta_b = sha2_hash_lower_32bit(&(poly_sx.evaluate(2, p)).to_string());
        let eta_c = sha2_hash_lower_32bit(&(poly_sx.evaluate(3, p)).to_string());

        let etas = &[eta_a, eta_b, eta_c];

        // From wiki: [https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/3-proof-generation-phase#id-3-5-2-ahp-proof]
        //             Step 6
        // let alpha = 10;
        // let eta_a = 2;
        // let eta_b = 30;
        // let eta_c = 100;

        // Compute polynomial for ∑ ηz(x)
        // let sigma_eta_z_x = Poly::new(vec![eta_a]) * &poly_z_hat_a
        //     + Poly::new(vec![eta_b]) * &poly_z_hat_b
        //     + Poly::new(vec![eta_c]) * &poly_z_hat_c;
        
        let sigma_eta_z_x = poly_add_many!(p,
            poly_fmath::mul_by_number(&poly_z_hat_a, eta_a, p),
            poly_fmath::mul_by_number(&poly_z_hat_b, eta_b, p),
            poly_fmath::mul_by_number(&poly_z_hat_c, eta_c, p)
        );

        println_dbg!("sigma_eta_z_x");
        println_dbg!("{}", sigma_eta_z_x);

        // Compute polynomial for r(α,x) ∑ ηM(z^M(x))
        let poly_r = poly_func_u(Some(alpha), None, set_h.len(), p);
        println_dbg!("poly_r:");
        println_dbg!("{}", poly_r);

        println_dbg!("r(alpha_2 , x) ∑_m [η_M z^_M(x)]:");
        println_dbg!("{}", poly_fmath::mul(&poly_r, &sigma_eta_z_x, p));

        // r(α,x) * ∑_m [η_M ​z^M​(x)]
        let sum_1 = poly_fmath::mul(&poly_r, &sigma_eta_z_x, p);
        // let sum_1 = poly_multiply(&poly_r, &sigma_eta_z_x, class_data.g);
        // assert_eq!(sum_12, sum_1, "g: {}", class_data.g);
        println_dbg!("sum_1: ");
        println_dbg!("{}", sum_1);

        // Compute polynomial for Z^(x)
        let tmp = poly_fmath::mul(&poly_w_hat, &van_poly_vh1, p);
        let poly_z_hat_x = poly_fmath::add(&tmp, &poly_x_hat, p);

        println_dbg!("z_hat: ");
        println_dbg!("{}", poly_z_hat_x);

        let (r_a_kx, r_b_kx, r_c_kx) =
            Self::calculate_r_polynomials_with_alpha(&points_px, alpha, &set_h, class_data.g, p);

        // ∑_m [η_M r_M(α,x)] * z^(x)
        // FIXME: Check here
        let mut sum_2 = FPoly::zero();
        for (poly, eta) in [&r_a_kx, &r_b_kx, &r_c_kx].iter().zip(etas.iter()) {
            let tmp = poly_fmath::mul_by_number(poly, *eta, p);
            sum_2 = poly_fmath::add(&sum_2, &tmp, p);
        }

        let sum_2 = poly_fmath::mul(&sum_2, &poly_z_hat_x, p);

        // Sum Check Protocol Formula:
        // s(x) + r(α,x) * ∑_m [η_M ​z^M​(x)] - ∑_m [η_M r_M(α,x)] * z^(x)
        let tmp = poly_fmath::add(&poly_sx, &sum_1, p);
        let poly_scp = poly_fmath::sub(&tmp, &sum_2, p);

        println_dbg!("scp: ");
        println_dbg!("{}", poly_scp);

        let div_res = poly_fmath::div(&poly_scp, &van_poly_vhx, p);
        let h_1x = div_res.0;
        println_dbg!("Poly h_1x: ");
        println_dbg!("{}", h_1x);

        let g_1x = poly_fmath::div(&div_res.1, &FPoly::one_x(), p).0;
        println_dbg!("Poly g_1x:");
        println_dbg!("{}", g_1x);

        // TODO: Random F - H
        let beta_1 = generate_beta_random(8, &poly_sx, &set_h, p);
        let beta_2 = generate_beta_random(9, &poly_sx, &set_h, p);

        // let beta_1 = 22);
        // let beta_2 = 80);


        // sigma_2
        let mut sigma_2 = 0;
        for (num, eta) in [r_a_kx.evaluate(beta_1, p), r_b_kx.evaluate(beta_1, p), r_c_kx.evaluate(beta_1, p)].iter().zip(etas.iter()) {
            let tmp = fmath::mul(*num, *eta, p);
            sigma_2 = fmath::add(sigma_2, tmp, p);
        }
        println_dbg!("sigma_2: {}", sigma_2);


        let (r_a_xk, r_b_xk, r_c_xk) =
            Self::calculate_r_polynomials_with_beta(&points_px, beta_1, &set_h, p);

        // r(alpha_2, x) ∑_m [​η_M ​M^(x,β1​)]
        let mut poly_sigma_2 = FPoly::zero();
        for (poly, eta) in [r_a_xk, r_b_xk, r_c_xk].iter().zip(etas.iter()) {
            let tmp = poly_fmath::mul_by_number(poly, *eta, p);
            poly_sigma_2 = poly_fmath::add(&poly_sigma_2, &tmp, p);
        }

        let poly_sigma_2 = poly_fmath::mul(&poly_r, &poly_sigma_2, p);

        println_dbg!("r(alpha_2, x) * ∑_m [η_M M^(x, β1)]: ");
        println_dbg!("{}", poly_sigma_2);

        let div_res = poly_fmath::div(&poly_sigma_2, &van_poly_vhx, p);
        let h_2x = div_res.0;
        println_dbg!("Poly h_2x: ");
        println_dbg!("{}", h_2x);

        let g_2x = poly_fmath::div(&div_res.1, &FPoly::one_x(), p).0;
        println_dbg!("Poly g_2x:");
        println_dbg!("{}", g_2x);

        // sigma_3
        let mut sigma_3 = 0;

        let polys_px = commitment_json.get_polys_px();

        // f_3x
        let poly_f_3x = Self::generate_poly_fx(
            &mut sigma_3,
            &polys_px,
            &van_poly_vhx,
            &vec![eta_a, eta_b, eta_c],
            &vec![beta_1, beta_2],
            &set_k,
            p
        );
        println_dbg!("poly_f_3x");
        println_dbg!("{}", poly_f_3x);
        println_dbg!("sigma_3: {}", sigma_3);

        let (pi_a, pi_b, pi_c) = Self::compute_polys_pi(beta_1, beta_2, &polys_px, p);
        let polys_pi = vec![&pi_a, &pi_b, &pi_c];

        println_dbg!("poly_pi_a");
        println_dbg!("{}", polys_pi[0]);
        println_dbg!("poly_pi_b");
        println_dbg!("{}", polys_pi[1]);
        println_dbg!("poly_pi_c");
        println_dbg!("{}", polys_pi[2]);

        // a(x)
        let poly_a_x = Self::generate_poly_ax(
            &polys_px,
            vec![beta_1, beta_2],
            &van_poly_vhx,
            vec![eta_a, eta_b, eta_c],
            &polys_pi,
            p
        );
        println_dbg!("poly_a_x");
        println_dbg!("{}", poly_a_x);

        // b(x)
        let poly_b_x = poly_fmath::mul(&poly_fmath::mul(polys_pi[0], polys_pi[1], p), &polys_pi[2], p);
        println_dbg!("poly_b_x");
        println_dbg!("{}", poly_b_x);

        let van_poly_vkx = vanishing_poly(&set_k, p);
        println_dbg!("van_poly_vkx");
        println_dbg!("{}", van_poly_vkx);

        let sigma_3_set_k = fmath::div(sigma_3, set_k.len() as u64, p);
        println_dbg!("sigma_3_set_k {}", sigma_3_set_k);

        let poly_f_3x = poly_fmath::sub(&poly_f_3x, &fpoly!(sigma_3_set_k), p);

        println_dbg!("poly_f_3x");
        println_dbg!("{}", poly_f_3x);

        let g_3x = poly_fmath::div(&poly_f_3x, &FPoly::one_x(), p).0;
        println_dbg!("g_3x");
        println_dbg!("{}", g_3x);

        let tmp_add = poly_fmath::add(&poly_f_3x, &fpoly!(sigma_3_set_k), p);
        let tmp_mul = poly_fmath::mul(&poly_b_x, &tmp_add, p);
        let tmp_sub = poly_fmath::sub(&poly_a_x, &tmp_mul, p);
        let h_3x = poly_fmath::div(&tmp_sub, &van_poly_vkx, p).0;

        println_dbg!("h_3x");
        println_dbg!("{}", h_3x);

        let polys_proof = [
            poly_w_hat,
            poly_z_hat_a,
            poly_z_hat_b,
            poly_z_hat_c,
            poly_h_0,
            poly_sx.clone(),
            g_1x,
            h_1x,
            g_2x,
            h_2x,
            g_3x,
            h_3x,
        ];

        // Print each variable with its name
        println_dbg!("poly_w_hat");
        println_dbg!("{}", polys_proof[0]);

        println_dbg!("poly_z_hat_a");
        println_dbg!("{}", polys_proof[1]);

        println_dbg!("poly_z_hat_b");
        println_dbg!("{}", polys_proof[2]);

        println_dbg!("poly_z_hat_c");
        println_dbg!("{}", polys_proof[3]);

        println_dbg!("poly_h_0");
        println_dbg!("{}", polys_proof[4]);

        println_dbg!("poly_sx");
        println_dbg!("{}", polys_proof[5]);

        println_dbg!("g_1x");
        println_dbg!("{}", polys_proof[6]);

        println_dbg!("h_1x");
        println_dbg!("{}", polys_proof[7]);

        println_dbg!("g_2x");
        println_dbg!("{}", polys_proof[8]);

        println_dbg!("h_2x");
        println_dbg!("{}", polys_proof[9]);

        println_dbg!("g_3x");
        println_dbg!("{}", polys_proof[10]);

        println_dbg!("h_3x");
        println_dbg!("{}", polys_proof[11]);

        // TODO:
        // let eta_values = [
        //     1),  // eta_w
        //     4),  // eta_z_a
        //     10), // eta_z_b
        //     8),  // eta_z_c
        //     32), // eta_h0
        //     45), // eta_s
        //     92), // eta_g1
        //     11), // eta_h1
        //     1),  // eta_g2
        //     5),  // eta_h2
        //     25), // eta_g3
        //     63), // eta_h3
        // ];

        let mut eta_values = vec![];
        for i in 10..=21 {
            eta_values.push(sha2_hash_lower_32bit(&poly_sx.evaluate(i, p).to_string()))
        }

        let poly_px = eta_values
            .iter()
            .enumerate()
            .map(|(i, &eta)| poly_fmath::mul_by_number(&polys_proof[i], eta, p))
            .fold(FPoly::zero(), |acc, poly| poly_fmath::add(&acc, &poly, p));

        println_dbg!("poly_px:");
        println_dbg!("{}", poly_px);

        // TODO:
        let z = sha2_hash_lower_32bit(&(poly_sx.evaluate(22, p).to_string()));
        // let z = 2);
        let val_y_p = poly_px.evaluate(z, p);
        println_dbg!("val_y_p {}", val_y_p);

        let mut poly_px_add = poly_px;
        poly_px_add.add_term(fmath::inverse_add(val_y_p, p), 0);
        let poly_x_z = FPoly::new(vec![1, fmath::inverse_add(z, p)]);

        let poly_qx = poly_fmath::div(&poly_px_add, &poly_x_z, p).0;
        println_dbg!("poly_qx");
        println_dbg!("{}", poly_qx);

        let val_commit_poly_qx = kzg::commit(&poly_qx, commitment_key, p);
        println_dbg!("val_commit_qx: {}", val_commit_poly_qx);

        let sigma = [sigma_1, sigma_2, sigma_3];

        let commit_x = compute_all_commitment(&polys_proof, commitment_key, p);
        println_dbg!("commit_x: {:?}", commit_x);

        let x_vec = &z_vec[1..numebr_t_zero];
        Self::create_proof(
            &polys_proof,
            &sigma,
            &commit_x,
            val_y_p,
            val_commit_poly_qx,
            &x_vec.to_vec(),
        )
    }

    /// Computes three polynomials used for ax
    pub fn compute_polys_pi(beta_1: u64, beta_2: u64, polys_px: &[FPoly], p: u64) -> (FPoly, FPoly, FPoly) {
        let poly_pi_a =
            poly_fmath::mul(&poly_fmath::sub(&fpoly!(beta_2), &polys_px[0], p), &(poly_fmath::sub(&fpoly!(beta_1), &polys_px[1], p)), p);
        let poly_pi_b =
            poly_fmath::mul(&poly_fmath::sub(&fpoly!(beta_2), &polys_px[3], p), &(poly_fmath::sub(&fpoly!(beta_1), &polys_px[4], p)), p);
        let poly_pi_c =
            poly_fmath::mul(&poly_fmath::sub(&fpoly!(beta_2), &polys_px[6], p), &(poly_fmath::sub(&fpoly!(beta_1), &polys_px[7], p)), p);

        (poly_pi_a, poly_pi_b, poly_pi_c)
    }

    /// Generates a random polynomial with specified degree and coefficient range
    fn generate_random_polynomial(degree: usize, coefficient_range: (u64, u64), p: u64) -> FPoly {
        assert!(coefficient_range.1 < p);
        let mut rng = rand::thread_rng();
        let mut tmp = 0;
        let coefficients: Vec<u64> = repeat_with(|| {
            // TODO: use random terms
            // let random_value = rng.gen_range(coefficient_range.0..=coefficient_range.1);
            let random_value = tmp;
            tmp = tmp + 1;
            random_value
        })
        .take(degree + 1) // +1 because degree is the highest power
        .collect();

        let mut rand_poly = FPoly::new(coefficients);
        rand_poly.trim();
        rand_poly
    }

    /// Creates a proof structure from provided polynomial and commitment data
    fn create_proof(
        polys_proof: &[FPoly],
        sigma: &[u64],
        commit_x: &[u64],
        val_y_p: u64,
        val_commit_poly_qx: u64,
        x_vec: &Vec<u64>,
    ) -> Box<[AHPData]> {
        let mut proof_data = Vec::new();

        // Add the first element
        proof_data.push(AHPData::Array(write_set(x_vec)));

        // Add Commit AHPData for commit_x
        proof_data.extend(
            commit_x
                .iter()
                .map(|commit| AHPData::Commit(*commit)),
        );

        // Add Sigma AHPData
        proof_data.extend(sigma.iter().map(|sigma| AHPData::Sigma(*sigma)));

        // Add Polynomial AHPData for polys_proof
        proof_data.extend(
            polys_proof
                .iter()
                .map(|poly| AHPData::Polynomial(write_term(poly))),
        );

        // Add Value AHPData
        proof_data.push(AHPData::Value(val_y_p));
        proof_data.push(AHPData::Value(val_commit_poly_qx));

        Box::from(proof_data)
    }

    /// Computes polynomial Fx
    fn generate_poly_fx(
        sigma_3: &mut u64,
        polys_px: &[FPoly],
        van_poly_vhx: &FPoly,
        eta: &Vec<u64>,
        beta: &Vec<u64>,
        set_k: &Vec<u64>,
        p: u64
    ) -> FPoly {
        let mut points_f_3: Vec<Point> = vec![];
        for k in set_k.iter() {
            let sig_a = sigma_m(
                &van_poly_vhx,
                &eta[0],
                &beta[0],
                &beta[1],
                k,
                &[&polys_px[0], &polys_px[1], &polys_px[2]],
                p
            );
            let sig_b = sigma_m(
                &van_poly_vhx,
                &eta[1],
                &beta[0],
                &beta[1],
                k,
                &[&polys_px[3], &polys_px[4], &polys_px[5]],
                p
            );
            let sig_c = sigma_m(
                &van_poly_vhx,
                &eta[2],
                &beta[0],
                &beta[1],
                k,
                &[&polys_px[6], &polys_px[7], &polys_px[8]],
                p
            );

            let sum = sig_a + sig_b + sig_c;
            *sigma_3 += sum;
            points_f_3.push((*k, sum));
        }
        interpolate(&points_f_3, p)
    }

    /// Generates polynomial based on input parameters
    fn generate_poly_ax(
        polys_px: &[FPoly],
        beta: Vec<u64>,
        van_poly_vhx: &FPoly,
        eta: Vec<u64>,
        poly_pi: &[&FPoly],
        p: u64
    ) -> FPoly {
        println_dbg!("eta: {:?}", eta);

        let val_vhx_beta_1 = van_poly_vhx.evaluate(beta[0], p);
        let val_vhx_beta_2 = van_poly_vhx.evaluate(beta[1], p);
        let beta_mul = val_vhx_beta_2 * val_vhx_beta_1;

        let poly_sigma_a = poly_fmath::mul_by_number(&polys_px[2], fmath::mul(eta[0], beta_mul, p), p);
        let poly_sigma_b = poly_fmath::mul_by_number(&polys_px[5], fmath::mul(eta[1], beta_mul, p), p);
        let poly_sigma_c = poly_fmath::mul_by_number(&polys_px[8], fmath::mul(eta[2], beta_mul, p), p);

        println_dbg!("poly_sigma_a");
        println_dbg!("{}", poly_sigma_a);
        println_dbg!("poly_sigma_b");
        println_dbg!("{}", poly_sigma_b);
        println_dbg!("poly_sigma_c");
        println_dbg!("{}", poly_sigma_c);

        poly_add_many!(p,
            poly_mul_many!(p, &poly_sigma_a, &poly_pi[1], &poly_pi[2]),
            poly_mul_many!(p, &poly_sigma_b, &poly_pi[0], &poly_pi[2]),
            poly_mul_many!(p, &poly_sigma_c, poly_pi[0], poly_pi[1])  
        )
    }

    /// Store in Json file
    pub fn store(&self, path: &str, proof_data: Box<[AHPData]>, class_number: u8, commitment_id: String) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        let proof_json = ProofGenerationJson::new(proof_data, class_number, commitment_id);
        serde_json::to_writer(writer, &proof_json)?;
        Ok(())
    }

    /// Restore Commitment from Json file
    pub fn restore(path: &str) -> Result<ProofGenerationJson> {
        read_json_file(path)
    }
}

/// JSON struct according to Witi (not complete)
/// More Info: [wiki](https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/3-proof-generation-phase#id-3-4-proof-json-file-format)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProofGenerationJson {
    pub class: u8,
    pub commitment_id: String,

    // #[serde(rename = "DeviceEncodedID")]
    // device_encoded_id: String,
    #[serde(rename = "Com1_AHP_x")]
    com1ahp: Vec<u64>,

    #[serde(rename = "Com2_AHP_x")]
    com2ahp: u64,

    #[serde(rename = "Com3_AHP_x")]
    com3ahp: u64,

    #[serde(rename = "Com4_AHP_x")]
    com4ahp: u64,

    #[serde(rename = "Com5_AHP_x")]
    com5ahp: u64,

    #[serde(rename = "Com6_AHP_x")]
    com6ahp: u64,

    #[serde(rename = "Com7_AHP_x")]
    com7ahp: u64,

    #[serde(rename = "Com8_AHP_x")]
    com8ahp: u64,

    #[serde(rename = "Com9_AHP_x")]
    com9ahp: u64,

    #[serde(rename = "Com10_AHP_x")]
    com10ahp: u64,

    #[serde(rename = "Com11_AHP_x")]
    com11ahp: u64,

    #[serde(rename = "Com12_AHP_x")]
    com12ahp: u64,

    #[serde(rename = "Com13_AHP_x")]
    com13ahp: u64,

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

    #[serde(rename = "P17AHP")]
    p17ahp: u64,
}

impl ProofGenerationJson {
    pub fn new(proof_data: Box<[AHPData]>, class_number: u8, commitment_id: String) -> Self {
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
            class: class_number,
            commitment_id,
            com1ahp: x_vec,
            com2ahp: commits[0],
            com3ahp: commits[1],
            com4ahp: commits[2],
            com5ahp: commits[3],
            com6ahp: commits[4],
            com7ahp: commits[5],
            com8ahp: commits[6],
            com9ahp: commits[7],
            com10ahp: commits[8],
            com11ahp: commits[9],
            com12ahp: commits[10],
            com13ahp: commits[11],
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
            p17ahp: values[1],
        }
    }

    /// Get vector X (Vector X is the first part of vector Z, where Z = [X, W, Y])
    pub fn get_x_vec(&self) -> Vec<u64> {
        let mut x: Vec<u64> = self.com1ahp.iter().map(|v| *v).collect();
        x.insert(0, 1);
        x
    }

    /// Get polynomials
    pub fn get_poly(&self, num: usize) -> FPoly {
        let this_poly = match num {
            0 => &self.p2ahp,
            1 => &self.p3ahp,
            2 => &self.p4ahp,
            3 => &self.p5ahp,
            4 => &self.p6ahp,
            5 => &self.p7ahp,
            6 => &self.p8ahp,
            7 => &self.p9ahp,
            8 => &self.p11ahp,
            9 => &self.p12ahp,
            10 => &self.p14ahp,
            11 => &self.p15ahp,
            _ => panic!(
                "Error: Invalid index {}. Expected a value between 0 and 11.",
                num
            ),
        };

        let poly_vec = this_poly
            .iter()
            .rev()
            .map(|&v| v)
            .collect::<Vec<u64>>();

        let mut poly = FPoly::new(poly_vec);
        poly.trim();
        poly
    }

    /// Get commits
    pub fn get_commits(&self, num: usize) -> u64 {
        *match num {
            0 => &self.com2ahp,
            1 => &self.com3ahp,
            2 => &self.com4ahp,
            3 => &self.com5ahp,
            4 => &self.com6ahp,
            5 => &self.com7ahp,
            6 => &self.com8ahp,
            7 => &self.com9ahp,
            8 => &self.com10ahp,
            9 => &self.com11ahp,
            10 => &self.com12ahp,
            11 => &self.com13ahp,
            _ => panic!(
                "Error: Invalid index {}. Expected a value between 0 and 11.",
                num
            ),
        }
    }

    /// Get sigma values
    pub fn get_sigma(&self, num: usize) -> u64 {
        match num {
            1 => self.p1ahp,
            2 => self.p10ahp,
            3 => self.p13ahp,
            _ => panic!("Invalid sigma number"),
        }
    }

    /// Get 1:p16ahp, and 2:p17ahp
    /// For more details, refer to the [documentation](https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/3-proof-generation-phase#id-3-3-proof-structure)
    pub fn get_value(&self, num: usize) -> u64 {
        match num {
            1 => self.p16ahp,
            2 => self.p17ahp,
            _ => panic!("Invalid value number"),
        }
    }
}
