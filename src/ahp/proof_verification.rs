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

use ark_ff::Field;
use rand::thread_rng;
use rand::Rng;
use rustnomial::Evaluable;
use rustnomial::FreeSizePolynomial;
use rustnomial::SizedPolynomial;

use crate::dsp_poly;
use crate::json_file::ClassDataJson;
use crate::kzg;
use crate::math::div_mod;
use crate::math::div_mod_val;
use crate::math::e_func;
use crate::math::poly_func_u;
use crate::math::generate_set;
use crate::math::interpolate;
use crate::math::vanishing_poly;
use crate::math::Mfp;
use crate::math::Poly;
use crate::math::P;
use crate::println_dbg;
use crate::to_bint;
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
    /// - `x_vec`: Vector of Mfp values
    ///
    /// # Returns
    /// Returns true if verification is successful, false otherwise
    pub fn verify(
        &self,
        (ck, vk): (&[Mfp], Mfp),
        class_data: ClassDataJson,
        polys_px: Vec<Poly>,
        x_vec: Vec<Mfp>,
        g: u64
    ) -> bool {
        let poly_sx = &self.data.get_poly(Polys::Sx as usize);
        let set_h_len = class_data.n as usize;
        let set_h = generate_set(set_h_len as u64, class_data);
        let set_k_len = class_data.m as usize;

        
        // Final random numbers must not be in set h
        let beta_1 = generate_beta_random(8, &poly_sx, &set_h);
        let beta_2 = generate_beta_random(9, &poly_sx, &set_h);
        // let beta_3 = Mfp::from(5);
        let beta_3 = Mfp::from(thread_rng().gen_range(1..1000));
        

        // TODO:
        // From wiki: [https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/3-proof-generation-phase#id-3-5-2-ahp-proof]
        //             Step 6
        let alpha = Mfp::from(sha2_hash_lower_32bit(&(poly_sx.eval(Mfp::from(0))).to_string()));
        let eta_a = Mfp::from(sha2_hash_lower_32bit(&(poly_sx.eval(Mfp::from(1))).to_string()));
        let eta_b = Mfp::from(sha2_hash_lower_32bit(&(poly_sx.eval(Mfp::from(2))).to_string()));
        let eta_c = Mfp::from(sha2_hash_lower_32bit(&(poly_sx.eval(Mfp::from(3))).to_string()));
        // println_dbg!("alpha: {:?} - {}", poly_sx.eval(Mfp::from(0)), alpha);

        // let alpha = Mfp::from(10);
        // let eta_a = Mfp::from(2);
        // let eta_b = Mfp::from(30);
        // let eta_c = Mfp::from(100);

        let z = Mfp::from(sha2_hash_lower_32bit(&poly_sx.eval(Mfp::from(22)).to_string()));
        // let z = Mfp::from(2);

        let beta = vec![beta_1, beta_2, beta_3];
        // let beta = vec![Mfp::from(22), Mfp::from(80), Mfp::from(5)];

        let eta = vec![eta_a, eta_b, eta_c];
        let t = (class_data.n_i + 1) as usize;

        // https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/4-proof-verification-phase#id-4-2-ahp-verify
        // All functions need to be executed for debugging purposes, hence they are written this way
        let mut res = true;
        res &= self.check_1(&polys_px, &beta, &eta, set_h_len, set_k_len);
        res &= self.check_2(&beta, alpha, set_h_len);
        res &= self.check_3(x_vec, alpha, &beta, &eta, &set_h, t);
        res &= self.check_4(&beta, set_h_len);
        res &= self.check_5((ck, vk), z, Mfp::from(g), &poly_sx);
        res
    }

    /// Checks the first verification equation
    ///
    /// # Parameters
    /// - `polys_px`: Vector of polynomials
    /// - `beta`: Array of Mfp values
    /// - `eta`: Array of Mfp values
    /// - `set_h_len`: Length of the set for h
    /// - `set_k_len`: Length of the set for k
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_1(
        &self,
        polys_px: &Vec<Poly>,
        beta: &[Mfp],
        eta: &[Mfp],
        set_h_len: usize,
        set_k_len: usize,
    ) -> bool {
        // Preparing equation values
        let van_poly_vkx = Self::vanishing_poly(set_k_len);
        let van_poly_vhx = Self::vanishing_poly(set_h_len);

        let (pi_a, pi_b, pi_c) = ProofGeneration::compute_polys_pi(beta[0], beta[1], polys_px);
        let polys_pi = vec![&pi_a, &pi_b, &pi_c];

        let poly_a_x = Self::generate_poly_ax(polys_px, beta, &van_poly_vhx, eta, &polys_pi);
        let poly_b_x = polys_pi[0] * polys_pi[1] * polys_pi[2];

        Self::check_equation_1(
            &self.data.get_poly(Polys::H3x as usize),
            &self.data.get_poly(Polys::G3x as usize),
            &van_poly_vkx,
            &poly_a_x,
            &poly_b_x,
            &beta[2],
            &self.data.get_sigma(3),
            set_k_len,
        )
    }

    /// Checks the second verification equation
    ///
    /// # Parameters
    /// - `beta`: Array of Mfp values
    /// - `alpha`: Mfp value
    /// - `set_h_len`: Length of the set for h
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_2(&self, beta: &[Mfp], alpha: Mfp, set_h_len: usize) -> bool {
        // Preparing equation values
        let van_poly_vhx = Self::vanishing_poly(set_h_len); // Vanishing polynomial for h
        let poly_r = poly_func_u(Some(alpha), None, set_h_len); // Compute polynomial r

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
        )
    }

    /// Checks the third verification equation
    ///
    /// # Parameters
    /// - `x`: Vector of Mfp values
    /// - `alpha`: Mfp value
    /// - `beta`: Array of Mfp values
    /// - `eta`: Array of Mfp values
    /// - `set_h`: Vector of Mfp values
    /// - `t_zero`: Index for the subset of H
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_3(
        &self,
        x: Vec<Mfp>,
        alpha: Mfp,
        beta: &[Mfp],
        eta: &[Mfp],
        set_h: &Vec<Mfp>,
        t_zero: usize,
    ) -> bool {
        // Preparing equation values

        let van_poly_vhx = Self::vanishing_poly(set_h.len()); // Vanishing polynomial for h
        let poly_r = poly_func_u(Some(alpha), None, set_h.len()); // Compute polynomial r
        let sum_1 = self.gen_poly_sigma(&eta, &poly_r); // Generate sigma polynomial
        let set_h_1 = &set_h[0..t_zero].to_vec(); // Subset of H

        let points = get_points_set(&x, set_h_1); // Get points for interpolation
        let poly_x_hat = interpolate(&points); // Interpolate polynomial

        // Compute the vanishing polynomial for the subset H
        let van_poly_vh1 = vanishing_poly(set_h_1);
        let poly_z_hat_x = &self.data.get_poly(Polys::WHat as usize) * &van_poly_vh1 + poly_x_hat; // Combine polynomials

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
        )
    }

    /// Checks the fourth verification equation
    ///
    /// # Parameters
    /// - `beta`: Array of Mfp values
    /// - `set_h_len`: Length of the set for h
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_4(&self, beta: &[Mfp], set_h_len: usize) -> bool {
        println_dbg!("equation 4 ======");
        // Preparing equation values
        let van_poly_vhx = Self::vanishing_poly(set_h_len); // Vanishing polynomial for h
        println_dbg!("van_poly_vhx: ");
        dsp_poly!(van_poly_vhx);

        let poly_ab_c = &self.data.get_poly(Polys::ZHatA as usize)
            * &self.data.get_poly(Polys::ZHatB as usize)
            - &self.data.get_poly(Polys::ZHatC as usize); // Compute polynomial A * B - C

        println_dbg!("poly_ab_c: ");
        dsp_poly!(poly_ab_c);
        
        let poly_h_0 = div_mod(&poly_ab_c, &van_poly_vhx); // Divide and get the result
        
        println_dbg!("poly_h_0: ");
        dsp_poly!(poly_h_0.0);

        // Ensure this division has no remainders
        assert!(poly_h_0.1.is_zero(), "Verify panic: The remainder of the division for poly_h_0 should be zero");

        // Check the fourth verification equation
        Self::check_equation_4(&poly_ab_c, &poly_h_0.0, &van_poly_vhx, &beta[0])
    }

    /// Checks the fifth verification equation
    ///
    /// # Parameters
    /// - `ck`: Array of commitment keys
    /// - `vk`: Verifying key
    /// - `z`: Mfp value
    /// - `g`: Mfp value
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_5(&self, (ck, vk): (&[Mfp], Mfp), z: Mfp, g: Mfp, poly_sx: &Poly) -> bool {
        // Preparing equation values
        // TODO: Replace with random values in the range (1..P)
        // let eta_values = [
        //     Mfp::from(1),  // eta_w
        //     Mfp::from(4),  // eta_z_a
        //     Mfp::from(10), // eta_z_b
        //     Mfp::from(8),  // eta_z_c
        //     Mfp::from(32), // eta_h0
        //     Mfp::from(45), // eta_s
        //     Mfp::from(92), // eta_g1
        //     Mfp::from(11), // eta_h1
        //     Mfp::from(1),  // eta_g2
        //     Mfp::from(5),  // eta_h2
        //     Mfp::from(25), // eta_g3
        //     Mfp::from(63), // eta_h3
        // ];

        let mut eta_values = vec![];
        for i in 10..=21 {
            eta_values.push(Mfp::from(sha2_hash_lower_32bit(&poly_sx.eval(Mfp::from(i)).to_string())))
        }

        // Compute polynomial px using eta values
        let poly_px = eta_values
            .iter()
            .enumerate()
            .map(|(i, &eta)| Poly::from(vec![eta]) * &self.data.get_poly(i).clone())
            .fold(Poly::zero(), |acc, poly| acc + poly);


        // Compute polynomial px using eta values
        let val_commit_poly_px = eta_values
            .iter()
            .enumerate()
            .map(|(i, &eta)| eta * self.data.get_commits(i).clone())
            .fold(Mfp::ZERO, |acc, com| acc + com);



        let val_y_p = poly_px.eval(z); // Evaluate polynomial at z

        let mut poly_px_add = poly_px;
        poly_px_add.add_term(-val_y_p, 0); // Adjust polynomial by subtracting evaluated value
        let poly_x_z = Poly::from(vec![Mfp::ONE, Mfp::from(-z)]); // Polynomial for division
        let poly_qx = div_mod(&poly_px_add, &poly_x_z).0; // Divide and get the result
        let val_commit_poly_qx = kzg::commit(&poly_qx, &ck); // Commit to polynomial qx

        // Check the fifth verification equation
        Self::check_equation_5(val_commit_poly_px, g, val_y_p, val_commit_poly_qx, vk, z)
    }

    #[inline]
    /// Generates the sigma polynomial using eta values and polynomial r
    ///
    /// # Parameters
    /// - `eta`: Array of Mfp values
    /// - `poly_r`: Polynomial r
    ///
    /// # Returns
    /// Returns the generated sigma polynomial
    fn gen_poly_sigma(&self, eta: &[Mfp], poly_r: &Poly) -> Poly {
        // Compute sigma polynomial using eta values and ZHat polynomials
        let sigma_eta_z_x = Poly::new(vec![eta[0]]) * &self.data.get_poly(Polys::ZHatA as usize)
            + Poly::new(vec![eta[1]]) * &self.data.get_poly(Polys::ZHatB as usize)
            + Poly::new(vec![eta[2]]) * &self.data.get_poly(Polys::ZHatC as usize);
        poly_r * sigma_eta_z_x // Multiply polynomial r with sigma polynomial
    }

    #[inline]
    /// Generates a vanishing polynomial of given length
    ///
    /// # Parameters
    /// - `len`: Length of the vanishing polynomial
    ///
    /// # Returns
    /// Returns the generated vanishing polynomial
    fn vanishing_poly(len: usize) -> Poly {
        let mut van = Poly::new(vec![-Mfp::ONE]); // Start with -1
        van.add_term(Mfp::ONE, len); // Add term for x^len
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
    /// - `beta_3`: Mfp value for beta3
    /// - `sigma_3`: Mfp value for sigma3
    /// - `set_k_len`: Length of the set for k
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_equation_1(
        h_3x: &Poly,
        g_3x: &Poly,
        van_poly_vkx: &Poly,
        ax: &Poly,
        bx: &Poly,
        beta_3: &Mfp,
        sigma_3: &Mfp,
        set_k_len: usize,
    ) -> bool {
        println_dbg!("h_3x: ");
        println_dbg!("g_3x: ");
        println_dbg!("van_poly_vkx:");
        dsp_poly!(van_poly_vkx);
        println_dbg!("ax: {:?}", ax);
        dsp_poly!(ax);
        println_dbg!("bx: {:?}", bx);
        dsp_poly!(bx);
        println_dbg!("beta_3: {:?}", beta_3);
        println_dbg!("sigma_3: {:?}", sigma_3);
        println_dbg!("set_k_len: {}", set_k_len);

        // Evaluate the left-hand side of the equation
        let eq11 = h_3x.eval(*beta_3) * van_poly_vkx.eval(*beta_3);

        // Evaluate the right-hand side of the equation
        let eq12 = ax.eval(*beta_3)
            - (bx.eval(*beta_3)
                * (*beta_3 * g_3x.eval(*beta_3)
                    + div_mod_val(*sigma_3, Mfp::from(set_k_len as u64))));

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
    /// - `beta_2`: Mfp value for beta2
    /// - `sigma_2`: Mfp value for sigma2
    /// - `sigma_3`: Mfp value for sigma3
    /// - `set_h_len`: Length of the set for h
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_equation_2(
        poly_r: &Poly,
        h_2x: &Poly,
        g_2x: &Poly,
        van_poly_vhx: &Poly,
        beta_2: &Mfp,
        sigma_2: &Mfp,
        sigma_3: &Mfp,
        set_h_len: usize,
    ) -> bool {
        // Print names of the arguments
        println_dbg!("poly_r:");
        dsp_poly!(poly_r);
        
        println_dbg!("h_2x:");
        dsp_poly!(h_2x);
        
        println_dbg!("g_2x:");
        dsp_poly!(g_2x);
        
        println_dbg!("van_poly_vhx:");
        dsp_poly!(van_poly_vhx);
        
        // Print Mfp values directly (assuming you have a way to print Mfp)
        println_dbg!("beta_2: {}", beta_2); // Replace with appropriate printing method for Mfp
        println_dbg!("sigma_2: {}", sigma_2); // Replace with appropriate printing method for Mfp
        println_dbg!("sigma_3: {}", sigma_3); // Replace with appropriate printing method for Mfp
        println_dbg!("set_h_len: {}", set_h_len);
        
        // Evaluate the left-hand side of the equation
        let eq21 = poly_r.eval(*beta_2) * sigma_3;

        println_dbg!("poly_r(beta_2)={} * sigma_3={}", poly_r.eval(*beta_2), sigma_3); 

        // Evaluate the right-hand side of the equation
        let eq22 = h_2x.eval(*beta_2) * van_poly_vhx.eval(*beta_2)
            + *beta_2 * g_2x.eval(*beta_2)
            + div_mod_val(*sigma_2, Mfp::from(set_h_len as u64));


        println_dbg!("h_2x(beta_2)={} *  van_hx(beta_2)={} + beta2={} * g_2x(beta_2)={} + sigma_2={} / set_h_len={}", h_2x.eval(*beta_2), van_poly_vhx.eval(*beta_2), beta_2, g_2x.eval(*beta_2), sigma_3, set_h_len);

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
    /// - `beta_1`: Mfp value for beta1
    /// - `sigma_1`: Mfp value for sigma1
    /// - `sigma_2`: Mfp value for sigma2
    /// - `set_h_len`: Length of the set for h
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_equation_3(
        poly_sx: &Poly,
        sum_1: &Poly,
        poly_z_hat_x: &Poly,
        h_1x: &Poly,
        g_1x: &Poly,
        van_poly_vhx: &Poly,
        beta_1: &Mfp,
        sigma_1: &Mfp,
        sigma_2: &Mfp,
        set_h_len: usize,
    ) -> bool {
        // Evaluate the left-hand side of the equation
        let eq31 =
            poly_sx.eval(*beta_1) + sum_1.eval(*beta_1) - *sigma_2 * poly_z_hat_x.eval(*beta_1);

        // Evaluate the right-hand side of the equation
        let eq32 = h_1x.eval(*beta_1) * van_poly_vhx.eval(*beta_1)
            + *beta_1 * g_1x.eval(*beta_1)
            + div_mod_val(*sigma_1, Mfp::from(set_h_len as u64));

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
    /// - `beta_1`: Mfp value for beta1
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    fn check_equation_4(
        poly_ab_c: &Poly,
        poly_h_0: &Poly,
        van_poly_vhx: &Poly,
        beta_1: &Mfp,
    ) -> bool {
        // Evaluate the left-hand side of the equation
        let eq41 = poly_ab_c.eval(*beta_1);

        // Evaluate the right-hand side of the equation
        let eq42 = poly_h_0.eval(*beta_1) * van_poly_vhx.eval(*beta_1);

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
    /// - `g`: Mfp value for g
    /// - `val_y_p`: Mfp value for y_p
    /// - `val_commit_poly_qx`: Commitment polynomial value for qx
    /// - `vk`: Mfp value for vk
    /// - `z`: Mfp value for z
    ///
    /// # Returns
    /// Returns true if the equation holds, false otherwise
    pub fn check_equation_5(
        val_commit_poly_px: Mfp,
        g: Mfp,
        val_y_p: Mfp,
        val_commit_poly_qx: Mfp,
        vk: Mfp,
        z: Mfp,
    ) -> bool {
        // Print input values for debugging
        println_dbg!("val_commit_poly_px: {val_commit_poly_px}, val_y_p: {val_y_p}, vk: {vk}, val_commit_poly_qx: {val_commit_poly_qx}");

        // Evaluate the first equation component
        let e_1 = e_func(
            val_commit_poly_px - Mfp::from(to_bint!(g) as u128 * to_bint!(val_y_p) as u128),
            g,
            g,
        );

        // Evaluate the second equation component
        let e_2 = e_func(
            val_commit_poly_qx,
            vk - Mfp::from(to_bint!(g) as u128 * to_bint!(z) as u128),
            Mfp::from(g),
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
    /// - `beta`: A slice of Mfp values representing beta values.
    /// - `van_poly_vhx`: The vanishing polynomial for vh.
    /// - `eta`: A slice of Mfp values representing eta values.
    /// - `poly_pi`: A vector of references to polynomial objects.
    ///
    /// # Returns
    /// Returns the generated polynomial ax.
    fn generate_poly_ax(
        polys_px: &[Poly],
        beta: &[Mfp],
        van_poly_vhx: &Poly,
        eta: &[Mfp],
        poly_pi: &[&Poly],
    ) -> Poly {
        // Evaluate the vanishing polynomial at beta[0] and beta[1]
        let val_vhx_beta_1 = van_poly_vhx.eval(beta[0]);
        println_dbg!("val_vhx_beta_1: {val_vhx_beta_1}");

        let val_vhx_beta_2 = van_poly_vhx.eval(beta[1]);
        println_dbg!("val_vhx_beta_2: {val_vhx_beta_2}");

        // Generate polynomial components based on eta and polys_px
        let poly_sig_a = Poly::from(vec![eta[0] * val_vhx_beta_2 * val_vhx_beta_1]) * &polys_px[2];
        let poly_sig_b = Poly::from(vec![eta[1] * val_vhx_beta_2 * val_vhx_beta_1]) * &polys_px[5];
        let poly_sig_c = Poly::from(vec![eta[2] * val_vhx_beta_2 * val_vhx_beta_1]) * &polys_px[8];

        // Combine the polynomial components with poly_pi
        poly_sig_a * (poly_pi[1] * poly_pi[2])
            + poly_sig_b * (poly_pi[0] * poly_pi[2])
            + poly_sig_c * (poly_pi[0] * poly_pi[1])
    }
}
