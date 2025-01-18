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

use rand::thread_rng;
use rand::Rng;
use crate::field::fmath;
use crate::fpoly;
use crate::json_file::ClassDataJson;
use crate::kzg;
use crate::math::e_func;
use crate::math::poly_func_u;
use crate::math::generate_set;
use crate::math::interpolate;
use crate::math::vanishing_poly;
use crate::mul_many;
use crate::polynomial::poly_fmath;
use crate::polynomial::FPoly;
use crate::println_dbg;
use crate::utils::generate_beta_random;
use crate::utils::get_points_set;
use crate::utils::sha2_hash_lower_32bit;

use super::proof_generation::Polys;
use super::proof_generation::ProofGeneration;
use super::proof_generation::ProofGenerationJson;

/// Struct for verification data
pub struct Verification {
    pub data: ProofGenerationJson, // Proof generation data
}

impl Verification {
    /// Creates a new `Verification` instance from proof generation data
    pub fn new(data: &ProofGenerationJson) -> Self {
        Self { data: data.clone() }
    }

    /// Verifies the proof using commitment and verifying keys
    ///
    /// # Parameters
    /// - `ck`: Commitment keys
    /// - `vk`: Verifying key
    /// - `class_data`: Class data for verification
    /// - `polys_px`: Polynomials for verification
    /// - `x_vec`: Vector of u64 values
    ///
    /// # Returns
    /// Returns true if verification is successful, false otherwise
    pub fn verify(
        &self,
        (ck, vk): (&[u64], u64),
        class_data: ClassDataJson,
        polys_px: Vec<FPoly>,
        x_vec: Vec<u64>,
        g: u64,
        p: u64
    ) -> bool {
        let poly_sx = &self.data.get_poly(Polys::Sx as usize);
        let set_h_len = class_data.n as usize;
        let set_h = generate_set(set_h_len as u64, class_data, p);
        let set_k_len = class_data.m as usize;

        
        // Generate a random number that is not present in the set h
        let beta_1 = generate_beta_random(8, &poly_sx, &set_h, p);
        let beta_2 = generate_beta_random(9, &poly_sx, &set_h, p);
        // let beta_3 = 5;
        let beta_3 = thread_rng().gen_range(1..1000);
        

        // TODO:
        // From wiki: [https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/3-proof-generation-phase#id-3-5-2-ahp-proof]
        //             Step 6
        let alpha = u64::from(sha2_hash_lower_32bit(&(poly_sx.evaluate(0, p)).to_string()));
        let eta_a = u64::from(sha2_hash_lower_32bit(&(poly_sx.evaluate(1, p)).to_string()));
        let eta_b = u64::from(sha2_hash_lower_32bit(&(poly_sx.evaluate(2, p)).to_string()));
        let eta_c = u64::from(sha2_hash_lower_32bit(&(poly_sx.evaluate(3, p)).to_string()));

        // let alpha = u64::from(10);
        // let eta_a = u64::from(2);
        // let eta_b = u64::from(30);
        // let eta_c = u64::from(100);

        let z = u64::from(sha2_hash_lower_32bit(&poly_sx.evaluate(22, p).to_string()));
        // let z = u64::from(2);

        let beta = vec![beta_1, beta_2, beta_3];
        // let beta = vec![u64::from(22), u64::from(80), u64::from(5)];

        let eta = vec![eta_a, eta_b, eta_c];
        let t = (class_data.n_i + 1) as usize;

        // https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/4-proof-verification-phase#id-4-2-ahp-verify
        // All functions need to be executed for debugging purposes, hence they are written this way
        let mut res = true;
        res &= self.check_1(&polys_px, &beta, &eta, set_h_len, set_k_len, p);
        res &= self.check_2(&beta, alpha, set_h_len, p);
        res &= self.check_3(x_vec, alpha, &beta, &eta, &set_h, t, p);
        res &= self.check_4(&beta, set_h_len, p);
        res &= self.check_5((ck, vk), z, u64::from(g), &poly_sx, p);
        res
    }

    /// Checks the first verification equation
    ///
    /// # Parameters
    /// - `polys_px`: Vector of polynomials
    /// - `beta`: Array of u64 values
    /// - `eta`: Array of u64 values
    /// - `set_h_len`: Length of the set for h
    /// - `set_k_len`: Length of the set for k
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_1(
        &self,
        polys_px: &Vec<FPoly>,
        beta: &[u64],
        eta: &[u64],
        set_h_len: usize,
        set_k_len: usize,
        p: u64
    ) -> bool {
        // Preparing equation values
        let van_poly_vkx = Self::vanishing_poly(set_k_len, p);
        let van_poly_vhx = Self::vanishing_poly(set_h_len, p);

        let (pi_a, pi_b, pi_c) = ProofGeneration::compute_polys_pi(beta[0], beta[1], polys_px, p);
        let polys_pi = vec![&pi_a, &pi_b, &pi_c];

        let poly_a_x = Self::generate_poly_ax(polys_px, beta, &van_poly_vhx, eta, &polys_pi, p);
        
        let poly_b_x = poly_fmath::mul(&poly_fmath::mul(&polys_pi[0], &polys_pi[1], p), &polys_pi[2], p);

        Self::check_equation_1(
            &self.data.get_poly(Polys::H3x as usize),
            &self.data.get_poly(Polys::G3x as usize),
            &van_poly_vkx,
            &poly_a_x,
            &poly_b_x,
            &beta[2],
            &self.data.get_sigma(3),
            set_k_len,
            p
        )
    }

    /// Checks the second verification equation
    ///
    /// # Parameters
    /// - `beta`: Array of u64 values
    /// - `alpha`: u64 value
    /// - `set_h_len`: Length of the set for h
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_2(&self, beta: &[u64], alpha: u64, set_h_len: usize, p: u64) -> bool {
        // Preparing equation values
        let van_poly_vhx = Self::vanishing_poly(set_h_len, p); // Vanishing polynomial for h
        let poly_r = poly_func_u(Some(alpha), None, set_h_len, p); // Compute polynomial r

        // Check the second verification equation
        Self::check_equation_2(
            &poly_r,
            &self.data.get_poly(Polys::H2x as usize),
            &self.data.get_poly(Polys::G2x as usize),
            &van_poly_vhx,
            &beta[1],
            &self.data.get_sigma(2),
            &self.data.get_sigma(3),
            set_h_len,
            p
        )
    }

    /// Checks the third verification equation
    ///
    /// # Parameters
    /// - `x`: Vector of u64 values
    /// - `alpha`: u64 value
    /// - `beta`: Array of u64 values
    /// - `eta`: Array of u64 values
    /// - `set_h`: Vector of u64 values
    /// - `t_zero`: Index for the subset of H
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_3(
        &self,
        x: Vec<u64>,
        alpha: u64,
        beta: &[u64],
        eta: &[u64],
        set_h: &Vec<u64>,
        t_zero: usize,
        p: u64
    ) -> bool {
        // Preparing equation values

        let van_poly_vhx = Self::vanishing_poly(set_h.len(), p); // Vanishing polynomial for h
        let poly_r = poly_func_u(Some(alpha), None, set_h.len(), p); // Compute polynomial r
        let sum_1 = self.gen_poly_sigma(&eta, &poly_r, p); // Generate sigma polynomial
        let set_h_1 = &set_h[0..t_zero].to_vec(); // Subset of H

        let points = get_points_set(&x, set_h_1); // Get points for interpolation
        let poly_x_hat = interpolate(&points, p); // Interpolate polynomial

        // Compute the vanishing polynomial for the subset H
        let van_poly_vh1 = vanishing_poly(set_h_1, p);
        let tmp_mul = poly_fmath::mul(&self.data.get_poly(Polys::WHat as usize), &van_poly_vh1, p);
        let poly_z_hat_x = poly_fmath::add(&tmp_mul, &poly_x_hat, p); // Combine polynomials

        println_dbg!("poly_z_hat_x\n{}", poly_z_hat_x);

        // Check the third verification equation
        Self::check_equation_3(
            &self.data.get_poly(Polys::Sx as usize),
            &sum_1,
            &poly_z_hat_x,
            &self.data.get_poly(Polys::H1x as usize),
            &self.data.get_poly(Polys::G1x as usize),
            &van_poly_vhx,
            &beta[0],
            &self.data.get_sigma(1),
            &self.data.get_sigma(2),
            set_h.len(),
            p
        )
    }

    /// Checks the fourth verification equation
    ///
    /// # Parameters
    /// - `beta`: Array of u64 values
    /// - `set_h_len`: Length of the set for h
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_4(&self, beta: &[u64], set_h_len: usize, p: u64) -> bool {
        println_dbg!("equation 4 ======");
        // Preparing equation values
        let van_poly_vhx = Self::vanishing_poly(set_h_len, p); // Vanishing polynomial for h
        println_dbg!("van_poly_vhx: {}", van_poly_vhx);

        let tmp_mul = poly_fmath::mul(&self.data.get_poly(Polys::ZHatA as usize), &self.data.get_poly(Polys::ZHatB as usize), p);
        let poly_ab_c = poly_fmath::sub(&tmp_mul, &self.data.get_poly(Polys::ZHatC as usize), p); // Compute polynomial A * B - C

        println_dbg!("poly_ab_c: {}", poly_ab_c);
        
        let poly_h_0 = poly_fmath::div(&poly_ab_c, &van_poly_vhx, p); // Divide and get the result
        
        println_dbg!("poly_h_0: {}", poly_h_0.0);

        // Ensure this division has no remainders
        assert!(poly_h_0.1.is_zero(), "Verify panic: The remainder of the division for poly_h_0 should be zero");

        // Check the fourth verification equation
        Self::check_equation_4(&poly_ab_c, &poly_h_0.0, &van_poly_vhx, &beta[0], p)
    }

    /// Checks the fifth verification equation
    ///
    /// # Parameters
    /// - `ck`: Array of commitment keys
    /// - `vk`: Verifying key
    /// - `z`: u64 value
    /// - `g`: u64 value
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_5(&self, (ck, vk): (&[u64], u64), z: u64, g: u64, poly_sx: &FPoly, p: u64) -> bool {
        // Preparing equation values
        // TODO: Replace with random values in the range (1..P)
        // let eta_values = [
        //     u64::from(1),  // eta_w
        //     u64::from(4),  // eta_z_a
        //     u64::from(10), // eta_z_b
        //     u64::from(8),  // eta_z_c
        //     u64::from(32), // eta_h0
        //     u64::from(45), // eta_s
        //     u64::from(92), // eta_g1
        //     u64::from(11), // eta_h1
        //     u64::from(1),  // eta_g2
        //     u64::from(5),  // eta_h2
        //     u64::from(25), // eta_g3
        //     u64::from(63), // eta_h3
        // ];

        let mut eta_values = vec![];
        for i in 10..=21 {
            eta_values.push(sha2_hash_lower_32bit(&poly_sx.evaluate(i, p).to_string()))
        }

        // Compute polynomial px using eta values
        let poly_px = eta_values
            .iter()
            .enumerate()
            .map(|(i, &eta)| poly_fmath::mul(&fpoly!(eta), &self.data.get_poly(i).clone(), p))
            .fold(FPoly::zero(), |acc, poly| poly_fmath::add(&acc, &poly, p));


        // Compute polynomial px using eta values
        let val_commit_poly_px = eta_values
            .iter()
            .enumerate()
            .map(|(i, &eta)| fmath::mul(eta, self.data.get_commits(i).clone(), p))
            .fold(0, |acc, com| fmath::add(acc, com, p));



        let val_y_p = poly_px.evaluate(z, p); // Evaluate polynomial at z

        let mut poly_px_add = poly_px;
        poly_px_add.add_term(fmath::inverse_add(val_y_p, p), 0); // Adjust polynomial by subtracting evaluated value
        let poly_x_z = fpoly!(1, u64::from(fmath::inverse_add(z, p))); // Polynomial for division
        let poly_qx = poly_fmath::div(&poly_px_add, &poly_x_z, p).0; // Divide and get the result
        let val_commit_poly_qx = kzg::commit(&poly_qx, &ck, p); // Commit to polynomial qx

        // Check the fifth verification equation
        Self::check_equation_5(val_commit_poly_px, g, val_y_p, val_commit_poly_qx, vk, z, p)
    }

    #[inline]
    /// Generates the sigma polynomial using eta values and polynomial r
    ///
    /// # Parameters
    /// - `eta`: Array of u64 values
    /// - `poly_r`: Polynomial r
    ///
    /// # Returns
    /// Returns the generated sigma polynomial
    fn gen_poly_sigma(&self, eta: &[u64], poly_r: &FPoly, p: u64) -> FPoly {
        // Compute sigma polynomial using eta values and ZHat polynomials
        let zhat_a_eta_1 = poly_fmath::mul_by_number(&self.data.get_poly(Polys::ZHatA as usize), eta[0], p);
        let zhat_b_eta_2 = poly_fmath::mul_by_number(&self.data.get_poly(Polys::ZHatB as usize), eta[1], p);
        let zhat_c_eta_3 = poly_fmath::mul_by_number(&self.data.get_poly(Polys::ZHatC as usize), eta[2], p);

        let sigma_eta_z_x = poly_fmath::add(&zhat_a_eta_1, &zhat_b_eta_2, p);
        let sigma_eta_z_x = poly_fmath::add(&sigma_eta_z_x, &zhat_c_eta_3, p);
        
        poly_fmath::mul(poly_r, &sigma_eta_z_x, p) // Multiply polynomial r with sigma polynomial
    }

    #[inline]
    /// Generates a vanishing polynomial of given length
    ///
    /// # Parameters
    /// - `len`: Length of the vanishing polynomial
    ///
    /// # Returns
    /// Returns the generated vanishing polynomial
    fn vanishing_poly(len: usize, p: u64) -> FPoly {
        // FIXME: Use normal case
        let mut van = fpoly!(p - 1); // Start with -1
        van.add_term(1, len); // Add term for x^len
        van // Return the vanishing polynomial
    }

    /// Checks the first verification equation
    ///
    /// # Parameters
    /// - `h_3x`: Polynomial h3
    /// - `g_3x`: Polynomial g3
    /// - `van_poly_vkx`: Vanishing polynomial for vk
    /// - `ax`: Polynomial a
    /// - `bx`: Polynomial b
    /// - `beta_3`: u64 value for beta3
    /// - `sigma_3`: u64 value for sigma3
    /// - `set_k_len`: Length of the set for k
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_equation_1(
        h_3x: &FPoly,
        g_3x: &FPoly,
        van_poly_vkx: &FPoly,
        ax: &FPoly,
        bx: &FPoly,
        beta_3: &u64,
        sigma_3: &u64,
        set_k_len: usize,
        p: u64
    ) -> bool {
        println_dbg!("h_3x: ");
        println_dbg!("g_3x: ");
        println_dbg!("van_poly_vkx: {}", van_poly_vkx);
        println_dbg!("ax: {}", ax);
        println_dbg!("bx: {}", bx);
        println_dbg!("beta_3: {:?}", beta_3);
        println_dbg!("sigma_3: {:?}", sigma_3);
        println_dbg!("set_k_len: {}", set_k_len);

        // Evaluate the left-hand side of the equation
        let eq11 = fmath::mul(h_3x.evaluate(*beta_3, p), van_poly_vkx.evaluate(*beta_3, p), p);

        // Evaluate the right-hand side of the equation

        // [ beta_3 * g_3(beta_3) + sigma_3 / n ] mod p
        let tmp_x = fmath::add(fmath::mul(*beta_3, g_3x.evaluate(*beta_3, p), p), fmath::div(*sigma_3, set_k_len as u64, p), p);
        // [ b(beta_3) * tmp_x ] mod p
        let tmp_y = fmath::mul(bx.evaluate(*beta_3, p), tmp_x, p);
        // [ a(beta_3) - tmp_y ] mod p
        let eq12 = fmath::sub(ax.evaluate(*beta_3, p), tmp_y, p);

        // Print evaluated values for debugging
        println_dbg!("------------------------------------");
        println_dbg!("eq11: {eq11}");
        println_dbg!("eq12: {eq12}");
        println_dbg!("------------------------------------");

        // Check if both sides of the equation are equal
        eq11 == eq12
    }

    /// Checks the second verification equation
    /// # Parameters
    /// - `poly_r`: Polynomial r(α, β2)
    /// - `h_2x`: Polynomial h2
    /// - `g_2x`: Polynomial g2
    /// - `van_poly_vhx`: Vanishing polynomial for vh
    /// - `beta_2`: u64 value for beta2
    /// - `sigma_2`: u64 value for sigma2
    /// - `sigma_3`: u64 value for sigma3
    /// - `set_h_len`: Length of the set for h
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_equation_2(
        poly_r: &FPoly,
        h_2x: &FPoly,
        g_2x: &FPoly,
        van_poly_vhx: &FPoly,
        beta_2: &u64,
        sigma_2: &u64,
        sigma_3: &u64,
        set_h_len: usize,
        p: u64
    ) -> bool {
        // Print names of the arguments
        println_dbg!("poly_r: {}", poly_r);
        println_dbg!("h_2x: {}", h_2x);
        println_dbg!("g_2x: {}", g_2x);
        println_dbg!("van_poly_vhx: {}", van_poly_vhx);    
        // Print u64 values directly (assuming you have a way to print u64)
        println_dbg!("beta_2: {}", beta_2); // Replace with appropriate printing method for u64
        println_dbg!("sigma_2: {}", sigma_2); // Replace with appropriate printing method for u64
        println_dbg!("sigma_3: {}", sigma_3); // Replace with appropriate printing method for u64
        println_dbg!("set_h_len: {}", set_h_len);
        
        // Evaluate the left-hand side of the equation
        // [ r(beta_2) * sigma_3 ] mod p
        let eq21 = fmath::mul(poly_r.evaluate(*beta_2, p), *sigma_3, p);
        println_dbg!("poly_r(beta_2)={} * sigma_3={}", poly_r.evaluate(*beta_2, p), sigma_3); 

        // Evaluate the right-hand side of the equation
        // [ h_2(beta_2) * vanishing_poly_h(beta_2) ] mod p
        let tmp_x = fmath::mul(h_2x.evaluate(*beta_2, p), van_poly_vhx.evaluate(*beta_2, p), p);
        // [ beta_2 * g(beta_2) ] mod p
        let tmp_y = fmath::mul(*beta_2, g_2x.evaluate(*beta_2, p), p); 
        // [ tmp_x + tmp_y + sigma_2 / n ] mod p
        let eq22 = fmath::add(fmath::add(tmp_x, tmp_y, p), fmath::div(*sigma_2, set_h_len as u64, p), p);


        println_dbg!("h_2x(beta_2)={} *  van_hx(beta_2)={} + beta2={} * g_2x(beta_2)={} + sigma_2={} / set_h_len={}", 
        h_2x.evaluate(*beta_2, p), van_poly_vhx.evaluate(*beta_2, p), beta_2, g_2x.evaluate(*beta_2, p), sigma_3, set_h_len);

        // Print evaluated values for debugging
        println_dbg!("------------------------------------");
        println_dbg!("eq21: {}", eq21);
        println_dbg!("eq22: {}", eq22);
        println_dbg!("------------------------------------");

        // Check if both sides of the equation are equal
        eq21 == eq22
    }

    /// Checks the third verification equation
    ///
    /// # Parameters
    /// - `poly_sx`: Polynomial s(β1)
    /// - `sum_1`: Polynomial representing the sum of ηM z^M for M in {A, B, C}
    /// - `poly_z_hat_x`: Polynomial z^(β1)
    /// - `h_1x`: Polynomial h1
    /// - `g_1x`: Polynomial g1
    /// - `van_poly_vhx`: Vanishing polynomial for vh
    /// - `beta_1`: u64 value for beta1
    /// - `sigma_1`: u64 value for sigma1
    /// - `sigma_2`: u64 value for sigma2
    /// - `set_h_len`: Length of the set for h
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_equation_3(
        poly_sx: &FPoly,
        sum_1: &FPoly,
        poly_z_hat_x: &FPoly,
        h_1x: &FPoly,
        g_1x: &FPoly,
        van_poly_vhx: &FPoly,
        beta_1: &u64,
        sigma_1: &u64,
        sigma_2: &u64,
        set_h_len: usize,
        p: u64
    ) -> bool {
        // Evaluate the left-hand side of the equation
        // [ sx(beta_1) + sum_1(beta_1) ] mod p
        let tmp_x = fmath::add(poly_sx.evaluate(*beta_1, p), sum_1.evaluate(*beta_1, p), p);
        // [ simgma_2 * z_hat(beta_1) ] mod p
        let tmp_y = fmath::mul(*sigma_2, poly_z_hat_x.evaluate(*beta_1, p), p);
        // [ tmp_x - tmp_y ] mod p
        let eq31 = fmath::sub(tmp_x, tmp_y, p);

        // Evaluate the right-hand side of the equation
        // [ h1(beta_1) * vanishing_poly_h(beta_1) ] mod p
        let tmp_x = fmath::mul(h_1x.evaluate(*beta_1, p), van_poly_vhx.evaluate(*beta_1, p), p);
        // [ beta_1 * g1(beta_1) ] mod p
        let tmp_y = fmath::mul(*beta_1, g_1x.evaluate(*beta_1, p), p);
        // [ tmp_x + tmp_y + sigma_1 / n ] mod p
        let eq32 = fmath::add(fmath::add(tmp_x, tmp_y, p) ,fmath::div(*sigma_1, set_h_len as u64, p), p);

        // Print evaluated values for debugging
        println_dbg!("------------------------------------");
        println_dbg!("eq31: {}", eq31);
        println_dbg!("eq32: {}", eq32);
        println_dbg!("------------------------------------");

        // Check if both sides of the equation are equal
        eq31 == eq32
    }

    /// Checks the fourth verification equation
    ///
    /// # Parameters
    /// - `poly_ab_c`: Polynomial representing z^A(β1)z^B(β1) - z^C(β1)
    /// - `poly_h_0`: Polynomial h0
    /// - `van_poly_vhx`: Vanishing polynomial for vh
    /// - `beta_1`: u64 value for beta1
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_equation_4(
        poly_ab_c: &FPoly,
        poly_h_0: &FPoly,
        van_poly_vhx: &FPoly,
        beta_1: &u64,
        p: u64
    ) -> bool {
        // Evaluate the left-hand side of the equation
        let eq41 = poly_ab_c.evaluate(*beta_1, p);

        // Evaluate the right-hand side of the equation
        // [ h0(beta_1) * vanishing_poly_h(beta_1) ] mod p
        let eq42 = fmath::mul(poly_h_0.evaluate(*beta_1, p), van_poly_vhx.evaluate(*beta_1, p), p)  ;

        // Print evaluated values for debugging
        println_dbg!("------------------------------------");
        println_dbg!("eq41: {}", eq41);
        println_dbg!("eq42: {}", eq42);
        println_dbg!("------------------------------------");

        // Check if both sides of the equation are equal
        eq41 == eq42
    }

    /// Checks the fifth verification equation
    ///
    /// # Parameters
    /// - `val_commit_poly_px`: Commitment polynomial value for px
    /// - `g`: u64 value for g
    /// - `val_y_p`: u64 value for y_p
    /// - `val_commit_poly_qx`: Commitment polynomial value for qx
    /// - `vk`: u64 value for vk
    /// - `z`: u64 value for z
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    pub fn check_equation_5(
        val_commit_poly_px: u64,
        g: u64,
        val_y_p: u64,
        val_commit_poly_qx: u64,
        vk: u64,
        z: u64,
        p: u64
    ) -> bool {
        // Print input values for debugging
        println_dbg!("val_commit_poly_px: {val_commit_poly_px}, val_y_p: {val_y_p}, vk: {vk}, val_commit_poly_qx: {val_commit_poly_qx}");

        // Evaluate the first equation component
        let tmp_x = fmath::mul(g, val_y_p, p);
        let e_1 = e_func(
            fmath::sub(val_commit_poly_px, tmp_x, p),
            g,
            g,
            p
        );

        // Evaluate the second equation component
        let tmp_x = fmath::mul(g, z, p);
        let e_2 = e_func(
            val_commit_poly_qx,
            fmath::sub(vk, tmp_x, p),
            g,
            p
        );


        // Print evaluated values for debugging
        println_dbg!("------------------------------------");
        println_dbg!("eq51: {}", e_1);
        println_dbg!("eq52: {}", e_2);
        println_dbg!("------------------------------------");

        // Check if both evaluated components are equal
        e_1 == e_2
    }

    /// Generates the polynomial ax based on the provided parameters.
    ///
    /// # Parameters
    /// - `polys_px`: A vector of polynomial objects.
    /// - `beta`: A slice of u64 values representing beta values.
    /// - `van_poly_vhx`: The vanishing polynomial for vh.
    /// - `eta`: A slice of u64 values representing eta values.
    /// - `poly_pi`: A vector of references to polynomial objects.
    ///
    /// # Returns
    /// Returns the generated polynomial ax.
    /// 
    fn generate_poly_ax(
        polys_px: &[FPoly],
        beta: &[u64],
        van_poly_vhx: &FPoly,
        eta: &[u64],
        poly_pi: &[&FPoly],
        p: u64
    ) -> FPoly {
        // Evaluate the vanishing polynomial at beta[0] and beta[1]
        let val_vhx_beta_1 = van_poly_vhx.evaluate(beta[0], p);
        println_dbg!("val_vhx_beta_1: {val_vhx_beta_1}");

        let val_vhx_beta_2 = van_poly_vhx.evaluate(beta[1], p);
        println_dbg!("val_vhx_beta_2: {val_vhx_beta_2}");

        // Generate polynomial components based on eta and polys_px
        let poly_sigma_a = poly_fmath::mul(&FPoly::new(vec![mul_many!(p, eta[0], val_vhx_beta_2, val_vhx_beta_1)]), &polys_px[2], p);
        let poly_sigma_b = poly_fmath::mul(&FPoly::new(vec![mul_many!(p, eta[1], val_vhx_beta_2, val_vhx_beta_1)]), &polys_px[5], p);
        let poly_sigma_c = poly_fmath::mul(&FPoly::new(vec![mul_many!(p, eta[2], val_vhx_beta_2, val_vhx_beta_1)]), &polys_px[8], p);

        // Combine the polynomial components with poly_pi
        let product_a = poly_fmath::mul(&poly_sigma_a, &poly_fmath::mul(poly_pi[1], poly_pi[2], p), p);
        let product_b = poly_fmath::mul(&poly_sigma_b, &poly_fmath::mul(poly_pi[0], poly_pi[2], p), p);
        let product_c = poly_fmath::mul(&poly_sigma_c, &poly_fmath::mul(poly_pi[0], poly_pi[1], p), p);

        let intermediate_sum = poly_fmath::add(&product_a, &product_b, p);
        let total_sum = poly_fmath::add(&intermediate_sum, &product_c, p);
        total_sum
    }
}


#[cfg(test)]
mod verification_test {
    use super::*;
    const P: u64 = 1678321;

    #[test]
    fn test_check_equation_1() {
        let h_3x = fpoly!(
            1166561, 211242, 719491, 1291747, 1004539, 1587800, 445828, 923361, 482361, 1414088,
            1262383, 649202, 1428829, 1314917, 819576, 176439, 529530, 889773, 1508275, 1265390,
            359766, 1069023, 827076, 1069827, 255061, 40786, 298118, 488293, 1171445, 964419,
            856225, 984307, 1171340, 458513, 981348, 1440839, 1575503, 1617853, 1153046, 556019,
            602043, 494902
        );

        let g_3x = fpoly!(1152011, 933053, 1057743, 1515370, 1622430, 1294320, 1371749);

        let van_poly_vkx = fpoly!(1, 0, 0, 0, 0, 0, 0, 0, 1678320);
        let ax = fpoly!(
            1380320, 1272264, 818428, 744142, 182712, 1064811, 638209, 1523792, 153665, 1212499,
            467434, 144563, 1374949, 1619234, 1017093, 542658, 1377186, 699412, 204645, 288090,
            616659, 798377, 1617672, 1616106, 926822, 1392773, 1284398, 185680, 1272257, 799621,
            1540098, 591807, 674132, 788077, 1276261, 966671);
        let bx = fpoly!(
            252141, 1197703, 1181603, 1269831, 1150367, 1627718, 1571241, 133515, 397458, 999779,
            526063, 796786, 887021, 735774, 986881, 256637, 438638, 1351186, 1164365, 1345817,
            1644884, 118568, 1358612, 318485, 1316244, 787780, 984694, 1035122, 603127, 8817,
            1631789, 1145574, 527614, 1597424, 501498, 66520, 77607, 1641059, 353268, 1194665,
            868091, 809427, 46652);

        let beta_3 = 105;
        let sigma_3 = 1532224;
        let set_k_len = 8;

        // True
        assert!(Verification::check_equation_1(
            &h_3x,
            &g_3x,
            &van_poly_vkx,
            &ax,
            &bx,
            &beta_3,
            &sigma_3,
            set_k_len,
            P
        ));


        let beta_3_random = 34;
        assert!(Verification::check_equation_1(
            &h_3x,
            &g_3x,
            &van_poly_vkx,
            &ax,
            &bx,
            &beta_3_random,
            &sigma_3,
            set_k_len,
            P
        ));


        // False 
        assert!(!Verification::check_equation_1(
            &h_3x,
            &g_3x,
            &van_poly_vkx,
            &ax,
            &bx,
            &beta_3,
            &sigma_3,
            set_k_len + 1,
            P
        ));

        let h_3x_false = fpoly!(
            1166561, 211242, 719491, 1291747, 1004539, 1587800, 445828, 923361, 482361, 1414088,
            1262383, 649202, 1428828, 1314917, 819576, 176439, 529530, 889773, 1508275, 1265390,
            359766, 1069023, 827076, 1069827, 255061, 40786, 298118, 488293, 1171445, 964419,
            856225, 984307, 1171340, 458513, 981348, 1440839, 1575503, 1617853, 1153046, 556019,
            602043, 494902);
        assert!(!Verification::check_equation_1(
            &h_3x_false,
            &g_3x,
            &van_poly_vkx,
            &ax,
            &bx,
            &beta_3,
            &sigma_3,
            set_k_len,
            P
        ));

        let g_3x_false = fpoly!(
            1152011, 933053, 1057743, 1515370, 1622431, 1294320, 1371749);        
        assert!(!Verification::check_equation_1(
            &h_3x,
            &g_3x_false,
            &van_poly_vkx,
            &ax,
            &bx,
            &beta_3,
            &sigma_3,
            set_k_len,
            P
        ));


        let bx_false = fpoly!(
            252141, 1197703, 1181603, 1269831, 1150367, 1627718, 1571241, 133515, 397458, 999779,
            526063, 796786, 887021, 735774, 986881, 256637, 438638, 1351186, 1164365, 1345817,
            1644884, 118568, 1358612, 318485, 1316244, 787780, 984694, 1035122, 603127, 8817,
            1631789, 1145574, 527614, 1597424, 501428, 66520, 77607, 1641059, 353268, 1194665,
            868091, 809427, 46652);
        assert!(!Verification::check_equation_1(
            &h_3x,
            &g_3x,
            &van_poly_vkx,
            &ax,
            &bx_false,
            &beta_3,
            &sigma_3,
            set_k_len,
            P
        ));


        assert!(!Verification::check_equation_1(
            &h_3x,
            &g_3x,
            &van_poly_vkx,
            &ax,
            &bx,
            &beta_3,
            &42134,
            set_k_len,
            P
        ));
    }

    #[test]
    fn test_check_equation_2() {
        let poly_r = fpoly!(
            1, 1022694, 223572, 1359854, 683162, 785980, 292059, 1233539, 1136243, 1396267,
            1453436, 178045, 1151298, 1137583, 617970, 620457, 1404120, 225112, 365195, 928237,
            416532, 54272, 1573298, 1117075, 1186955, 778853, 500024, 562524, 82239, 1309914,
            331153, 1469913, 764243, 1032547, 188270, 579297, 1288081
        );
        let h_2x = fpoly!(
            1527224, 202963, 1641460, 1532214, 8621, 202835, 1266475, 76428, 328846, 1604258,
            1180872, 592632, 1195514, 806757, 868521, 1619619, 128535, 1564868, 916923, 279171,
            416096, 1404119, 812682, 484163, 1631832, 1470950, 637064, 262279, 438265, 576315,
            762439, 715840, 1405895, 1614708, 1002178, 655300
        );
        let g_2x = fpoly!(
            281627, 1265132, 472682, 962130, 1236583, 478787, 947473, 1589344, 661195, 14957,
            12545, 1041724, 539652, 147504, 868543, 438050, 1644532, 484346, 670378, 64071, 23450,
            1139153, 729093, 1481929, 952885, 1215237, 77842, 319022, 535671, 758793, 941287,
            242315, 274582, 910701, 699049, 393904
        );
        let van_poly_vhx = fpoly!(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1678320);
        let beta_2: &u64 = &361480;
        let sigma_2: &u64 = &873445;
        let sigma_3: &u64 = &724859;
        let set_h_len: usize = 37;

        // True
        assert!(Verification::check_equation_2(
            &poly_r,
            &h_2x,
            &g_2x,
            &van_poly_vhx,
            beta_2,
            sigma_2,
            sigma_3,
            set_h_len,
            P
        ));

        // False
        let poly_r_false = fpoly!(
            1, 1022695, 223572, 1359854, 683162, 785980, 292059, 1233539, 1136243, 1396267,
            1453436, 178045, 1151298, 1137583, 617970, 620457, 1404120, 225112, 365195, 928237,
            416532, 54272, 1573298, 1117075, 1186955, 778853, 500024, 562524, 82239, 1309914,
            331153, 1469913, 764243, 1032547, 188270, 579297, 1288081
        );
        assert!(!Verification::check_equation_2(
            &poly_r_false,
            &h_2x,
            &g_2x,
            &van_poly_vhx,
            beta_2,
            sigma_2,
            sigma_3,
            set_h_len,
            P
        ));
        
        let h_2x_false = fpoly!(
            1527224, 202963, 1641460, 1532214, 8621, 202835, 1266475, 76428, 328846, 1604258,
            1180842, 592632, 1195514, 806757, 868521, 1619619, 128535, 1564868, 916923, 279171,
            416096, 1404119, 812682, 484163, 1631832, 1470950, 637064, 262279, 438265, 576315,
            762439, 715840, 1405895, 1614708, 1002178, 655300
        );
        assert!(!Verification::check_equation_2(
            &poly_r,
            &h_2x_false,
            &g_2x,
            &van_poly_vhx,
            beta_2,
            sigma_2,
            sigma_3,
            set_h_len,
            P
        ));

        assert!(!Verification::check_equation_2(
            &poly_r,
            &h_2x,
            &g_2x,
            &van_poly_vhx,
            beta_2,
            &(*sigma_2 + 23),
            sigma_3,
            set_h_len,
            P
        ));

    }

    #[test]
    fn test_check_equation_3() {
        let poly_sx = &fpoly!(
            1663444, 811894, 37326, 861434, 1337494, 151771, 719042, 1377667, 572145, 1421419,
            213525, 1644675, 568882, 264178, 35159, 1011191, 1362672, 431500, 363274, 46841,
            262501, 640453, 931996, 658114, 47214, 1032214, 1375957, 339799, 300005, 1266828,
            271975, 367873, 1584843, 884622, 536301, 1461142, 1181181, 893300, 1516894, 1205012,
            1040817, 1140682, 408577, 561405, 208250, 1264230, 1503124, 1060605, 678989, 881484,
            650257, 1330285, 203834, 375069, 1285245, 1545405, 1606446, 472616, 1180729, 610077,
            393302, 723388, 990490, 1074477, 929029, 749494, 493421, 1170874, 754701, 624803,
            265812, 446578, 696761, 504846, 676001, 1585382
        );
        let sum_1 = &fpoly!(
            421607, 148036, 375890, 1466967, 1143242, 273354, 1331862, 1582727, 1601224, 90056,
            252534, 300124, 132933, 1289887, 622251, 1300810, 59373, 1338464, 1189845, 55992,
            928138, 766688, 697571, 1248719, 1509176, 1608203, 50574, 18181, 240839, 354221,
            532449, 1405880, 282149, 1154187, 367542, 1488803, 1007425, 1562587, 1237979, 1642415,
            1330105, 1411920, 405521, 316873, 951528, 18252, 557073, 690220, 1004634, 80522, 86907,
            1388766, 882514, 365582, 1554060, 461445, 1517614, 347528, 664656, 1083077, 1300262,
            1196032, 936930, 335878, 556562, 924938, 425872, 829241, 1306973, 1113903, 746810,
            226387, 1016548, 446480, 857039
        );
        let poly_z_hat_x = &fpoly!(
            1136303, 1053035, 1367307, 1104622, 1439496, 1106912, 1511145, 141021, 882468, 1194877,
            1177453, 245271, 896501, 556078, 745354, 293367, 517068, 756007, 933860, 245570,
            236901, 644375, 172645, 487007, 399049, 544277, 1490550, 1242825, 555934, 524524,
            297726, 187936, 137009, 347790, 1102826, 1080841, 881165, 128367, 765996
        );
        let h_1x = &fpoly!(
            1663444, 710965, 546303, 451439, 600448, 875160, 561051, 782426, 1166384, 813976,
            592962, 932434, 1597872, 184421, 887521, 46831, 591714, 258512, 231927, 820779, 578601,
            816173, 1478343, 1295585, 590308, 754018, 803702, 174913, 672164, 1327789, 9367,
            1141014, 1488424, 1313754, 1332806, 712382, 1121375, 412645, 536355
        );
        let g_1x = &fpoly!(
            83041, 96327, 175025, 761747, 1446337, 1571249, 450295, 499866, 209152, 1278090,
            1286341, 1665663, 1458683, 750831, 1275759, 1491384, 664268, 69561, 459147, 1285555,
            123531, 988921, 1396380, 36050, 878768, 1160828, 1110491, 1505973, 965970, 904968,
            801915, 21991, 1112999, 915315, 51587, 809527
        );
        let van_poly_vhx = &fpoly!(
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 1678320
        );
        let beta_1 = &577150;
        let sigma_1 = &488684;
        let sigma_2 = &686138;
        let set_h_len = 37;

        // True
        assert!(Verification::check_equation_3(
            poly_sx,
            sum_1,
            poly_z_hat_x,
            h_1x,
            g_1x,
            van_poly_vhx,
            beta_1,
            sigma_1,
            sigma_2,
            set_h_len,
            P
        ));

        // False
        let sum_1_false = &fpoly!(
            421607, 148036, 375890, 1466967, 1143242, 273354, 1331862, 1582727, 1601224, 90056,
            252534, 300124, 132933, 1289887, 622251, 1300810, 59373, 1338464, 1189845, 55992,
            928138, 766688, 697571, 1248719, 1509176, 1608203, 50574, 18181, 240839, 354221,
            532449, 1405880, 282149, 1154187, 367542, 1488803, 1007425, 1562587, 1237979, 1642415,
            1330105, 1411920, 405521, 316873, 951528, 18252, 557073, 690220, 1004634, 80522, 86907,
            1388766, 882514, 365582, 1554060, 461445, 1517614, 347528, 664656, 1083077, 1300262,
            1196032, 936930, 335878, 199862, 924938, 425872, 829241, 1306973, 1113903, 746810,
            226387, 1016548, 446480, 857039
        );
        assert!(!Verification::check_equation_3(
            poly_sx,
            sum_1_false,
            poly_z_hat_x,
            h_1x,
            g_1x,
            van_poly_vhx,
            beta_1,
            sigma_1,
            sigma_2,
            set_h_len,
            P
        ));
    }
    
    #[test]
    fn test_check_equation_4() {
        let poly_ab_c = &fpoly!(
            1596389, 32096, 1284991, 1596091, 1397885, 1531245, 241201, 1537643, 1038867, 48036,
            282310, 1377705, 239157, 651985, 220220, 921601, 1212152, 1184488, 264303, 1389649,
            155686, 382416, 2004, 155101, 577944, 543069, 1659084, 1155952, 1092891, 1036266,
            1525649, 875997, 1129813, 1249919, 532367, 1506558, 405537, 594859, 1213891, 62417,
            82230, 280436, 147076, 1437120, 140678, 639454, 1630285, 1396011, 300616, 1439164,
            1026336, 1458101, 756720, 466169, 493833, 1414018, 288672, 1522635, 1295905, 1676317,
            1523220, 1100377, 1135252, 19237, 522369, 585430, 642055, 152672, 802324, 548508,
            428402, 1145954, 171763, 1272784, 1165394, 432334, 330913
        );
        let poly_h_0 = &fpoly!(
            1596389, 32096, 1284991, 1596091, 1397885, 1531245, 241201, 1537643, 1038867, 48036,
            282310, 1377705, 239157, 651985, 220220, 921601, 1212152, 1184488, 264303, 1389649,
            155686, 382416, 2004, 155101, 577944, 543069, 1659084, 1155952, 1092891, 1036266,
            1525649, 875997, 1129813, 1249919, 532367, 1506558, 405537, 512927, 1245987, 1347408
        );
        let van_poly_vhx = &fpoly!(
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 1678320
        );
        let beta_1 = &577150;

        // True
        assert!(Verification::check_equation_4(
            poly_ab_c,
            poly_h_0,
            van_poly_vhx,
            beta_1,
            P
        ));

        let beta_1_another = &57149;
        assert!(Verification::check_equation_4(
            poly_ab_c,
            poly_h_0,
            van_poly_vhx,
            beta_1_another,
            P
        ));


        // False
        let poly_h_0_false = &fpoly!(
            1596389, 32096, 1284991, 1596091, 1397885, 1531245, 241201, 1537643, 1038867, 48036,
            282310, 1377705, 239157, 651985, 220220, 921601, 1212152, 1184488, 264303, 1389649,
            155686, 382416, 19198, 651108, 195114, 191122, 971321, 7797199, 11497197, 12100266,
            1525649, 875997, 1129813, 1249919, 532367, 1506558, 405537, 512927, 1245987, 1347408
        );
        assert!(!Verification::check_equation_4(
            poly_ab_c,
            poly_h_0_false,
            van_poly_vhx,
            beta_1,
            P
        ));


        let poly_ab_c_false = &fpoly!(
            1596389, 32096, 1284991, 1596091, 1397885, 1531245, 241201, 1537643, 1038867, 48036,
            282310, 1377705, 239157, 651985, 220220, 921601, 1212152, 1184488, 264303, 1389649,
            155686, 382416, 2004, 155101, 577944, 543069, 1659084, 1155952, 1092891, 1036266,
            1525649, 875997, 1129813, 1249919, 532367, 1506558, 405537, 594859, 1213891, 62417,
            82230, 280436, 147075, 1437120, 140678, 639454, 1630285, 1396011, 300616, 1439164,
            1026336, 1458101, 756720, 466169, 493833, 1414018, 288672, 1522635, 1295905, 1676317,
            1523220, 1100377, 1135252, 19237, 522369, 585430, 642055, 152672, 802324, 548508,
            428402, 1145954, 171763, 1272784, 1165394, 432334, 330913
        );
        assert!(!Verification::check_equation_4(
            poly_ab_c_false,
            poly_h_0,
            van_poly_vhx,
            beta_1,
            P
        ));
    }

    #[test]
    fn test_check_equation_5() {
        let val_commit_poly_px = 1226529;
        let g = 11;
        let val_y_p = 311048;
        let val_commit_poly_qx = 714628;
        let vk = 1309;
        let z = 1536867;

        // True
        assert!(Verification::check_equation_5(val_commit_poly_px, g, val_y_p, val_commit_poly_qx, vk, z, P));
        
        // False
        assert!(!Verification::check_equation_5(val_commit_poly_px + 1, g, val_y_p, val_commit_poly_qx, vk, z, P));
        assert!(!Verification::check_equation_5(val_commit_poly_px, g - 1, val_y_p, val_commit_poly_qx, vk, z, P));
        assert!(!Verification::check_equation_5(val_commit_poly_px, g, val_y_p + 2, val_commit_poly_qx, vk, z, P));
        assert!(!Verification::check_equation_5(val_commit_poly_px, g, val_y_p, val_commit_poly_qx - 3, vk, z, P));
        assert!(!Verification::check_equation_5(val_commit_poly_px, g, val_y_p, val_commit_poly_qx, vk + 4, z, P));
        assert!(!Verification::check_equation_5(val_commit_poly_px, g, val_y_p, val_commit_poly_qx, vk, z + 7, P));
    }
}
