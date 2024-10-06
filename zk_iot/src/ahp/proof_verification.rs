use ark_ff::Field;
use rustnomial::{Evaluable, FreeSizePolynomial};

use crate::{
    dsp_poly,
    math::{div_mod, div_mod_val, e_func, exp_mod, func_u, lagrange_interpolate, vanishing_poly, Mfp, Poly},
    to_bint, utils::{get_points_set, mat_to_vec},
};

use super::{commitment::Commitment, proof_generation::AHPData};

pub struct Verification {
    pub data: Box<[AHPData]>,
}

impl Verification {
    pub fn new(data: Box<[AHPData]>) -> Self {
        Self { data }
    }

    fn get_value(data: &AHPData) -> Mfp {
        match data {
            AHPData::Commit(val) | AHPData::Value(val) => Mfp::from(*val),
            _ => panic!("Unexpected AHPData variant"),
        }
    }

    fn get_poly(data: &AHPData) -> Poly {
        if let AHPData::Polynomial(poly) = data {
            Poly::new(
                poly.iter()
                    .rev()
                    .map(|&t| Mfp::from(t))
                    .collect::<Vec<Mfp>>(),
            )
        } else {
            panic!("Unexpected AHPData variant")
        }
    }

    pub fn verify(&self, commitment: &Commitment, vk: Mfp) -> bool {
        let poly_sx = Self::get_poly(&self.data[18]);
        // TODO:
        // From wiki: [https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/3-proof-generation-phase#id-3-5-2-ahp-proof]
        //             Step 6
        // let alpha = Mfp::from(sha2_hash(&(poly_sx.eval(Mfp::from(0))).to_string()));
        // let beta_1 = poly_sx.eval(Mfp::from(1));
        // let beta_2 = poly_sx.eval(Mfp::from(2));
        // let beta_3 = poly_sx.eval(Mfp::from(3));
        // let alpha = Mfp::from(sha2_hash(&(poly_sx.eval(Mfp::from(0))).to_string()));
        // let eta_a = Mfp::from(sha2_hash(&(poly_sx.eval(Mfp::from(1))).to_string()));
        // let eta_b = Mfp::from(sha2_hash(&(poly_sx.eval(Mfp::from(2))).to_string()));
        // let eta_c = Mfp::from(sha2_hash(&(poly_sx.eval(Mfp::from(3))).to_string()));

        let set_h_len = commitment.set_h.len();
        let set_k_len = commitment.set_k.len();
        // Randoms:
        let alpha = Mfp::from(10);
        let beta = vec![Mfp::from(22), Mfp::from(80), Mfp::from(5)];
        let eta = vec![Mfp::from(2), Mfp::from(30), Mfp::from(100)];

        // Polynomials
        let van_poly_vkx = Self::vanishing_poly(set_k_len);
        let van_poly_vhx = Self::vanishing_poly(set_h_len);


        let poly_r = func_u(Some(alpha), None, set_h_len);
        let sum_1 = Self::gen_poly_sigma(&eta, &self.data, &poly_r);
        let poly_ab_c = &Self::get_poly(&self.data[14]) * &Self::get_poly(&self.data[15])
            - &Self::get_poly(&self.data[16]);
        let poly_h_0 = div_mod(&poly_ab_c, &van_poly_vhx).0;


        // let poly_z_hat_x = &poly_w_hat * &van_poly_vh1 + poly_x_hat;
        
        self.check_1(&commitment, &beta, &eta) &&
        self.check_2(&beta, alpha, set_h_len) &&
        self.check_3(&commitment, alpha, &beta, &eta, set_h_len) &&
        self.check_4(&beta, set_h_len)
        // && Self::check_equation_5(val_com_p, Mfp::from(g), val_y_p, val_commit_poly_qx, vk, z)
    }

    fn check_1(&self, commitment: &Commitment, beta: &[Mfp], eta: &[Mfp]) -> bool {
        let set_h_len = commitment.set_h.len();
        let set_k_len = commitment.set_k.len();
        let van_poly_vkx = Self::vanishing_poly(set_k_len);
        let van_poly_vhx = Self::vanishing_poly(set_h_len);

        let poly_pi_a = (Poly::from(vec![beta[1]]) - &commitment.polys_px[0])
            * (Poly::from(vec![beta[0]]) - &commitment.polys_px[1]);
        let poly_pi_b = (Poly::from(vec![beta[1]]) - &commitment.polys_px[3])
            * (Poly::from(vec![beta[0]]) - &commitment.polys_px[4]);
        let poly_pi_c = (Poly::from(vec![beta[1]]) - &commitment.polys_px[6])
            * (Poly::from(vec![beta[0]]) - &commitment.polys_px[7]);
        let polys_pi = vec![&poly_pi_a, &poly_pi_b, &poly_pi_c];

        let poly_a_x = Self::gen_poly_ax(&commitment, beta, &van_poly_vhx, eta, &polys_pi);
        let poly_b_x = polys_pi[0] * polys_pi[1] * polys_pi[2];

        Self::check_equation_1(
            &Self::get_poly(&self.data[26]),
            &Self::get_poly(&self.data[25]),
            &van_poly_vkx,
            &poly_a_x,
            &poly_b_x,
            &beta[2],
            &Self::get_value(&self.data[24]),
            set_k_len,
        )
    }


    fn check_2(&self, beta: &[Mfp], alpha: Mfp, set_h_len: usize) -> bool {
        let van_poly_vhx = Self::vanishing_poly(set_h_len);
        let poly_r = func_u(Some(alpha), None, set_h_len);

        Self::check_equation_2(
            &poly_r,
            &Self::get_poly(&self.data[23]),
            &Self::get_poly(&self.data[22]),
            &van_poly_vhx,
            &beta[1],
            &Self::get_value(&self.data[21]),
            &Self::get_value(&self.data[24]),
            set_h_len,
        )
    }


    fn check_3(&self, commitment: &Commitment, alpha: Mfp, beta: &[Mfp], eta: &[Mfp], set_h_len: usize) -> bool {
        let van_poly_vhx = Self::vanishing_poly(set_h_len);
        let poly_r = func_u(Some(alpha), None, set_h_len);
        let sum_1 = Self::gen_poly_sigma(&eta, &self.data, &poly_r);
        let set_h_1 = &commitment.set_h[0..commitment.numebr_t_zero].to_vec(); // H[>∣x∣]
        // Interpolate polynomial for x^(h) over the subset H[>∣x∣]
        let z_vec = &mat_to_vec(&commitment.matrices.z);
        let points = get_points_set(&z_vec[0..commitment.numebr_t_zero], set_h_1);
        let poly_x_hat = lagrange_interpolate(&points);
        // Interpolate polynomial w(h) over the subset H[<=∣x∣]
        // Compute the vanishing polynomial for the subset H[<=∣x∣]
        let van_poly_vh1 = vanishing_poly(set_h_1);
        let poly_z_hat_x =  Self::get_poly(&self.data[13]) * &van_poly_vh1 + poly_x_hat;

        Self::check_equation_3(
            &Self::get_poly(&self.data[18]),
            &sum_1,
            &poly_z_hat_x,
            &Self::get_poly(&self.data[20]),
            &Self::get_poly(&self.data[19]),
            &van_poly_vhx,
            &beta[0],
            &Self::get_value(&self.data[12]),
            &Self::get_value(&self.data[21]),
            set_h_len,
        ) && 
        false
    }


    fn check_4(&self, beta: &[Mfp], set_h_len: usize) -> bool {
        let van_poly_vhx = Self::vanishing_poly(set_h_len);
        let poly_ab_c = &Self::get_poly(&self.data[14]) * &Self::get_poly(&self.data[15])
            - &Self::get_poly(&self.data[16]);
        let poly_h_0 = div_mod(&poly_ab_c, &van_poly_vhx).0;
        Self::check_equation_4(&poly_ab_c, &poly_h_0, &van_poly_vhx, &beta[0])
    }


    fn check_5() -> bool {
        
        false
    }

    #[inline]
    fn gen_poly_sigma(eta: &[Mfp], data: &[AHPData], poly_r: &Poly) -> Poly {
        let sigma_eta_z_x = Poly::new(vec![eta[0]]) * &Self::get_poly(&data[14])
            + Poly::new(vec![eta[1]]) * &Self::get_poly(&data[15])
            + Poly::new(vec![eta[2]]) * &Self::get_poly(&data[16]);
        poly_r * sigma_eta_z_x
    }

    #[inline]
    fn vanishing_poly(len: usize) -> Poly {
        let mut van = Poly::new(vec![-Mfp::ONE]);
        van.add_term(Mfp::ONE, len);
        van
    }

    // h3​(β3​)vK​(β3​)=a(β3​)−b(β3​)(β3​g3​(β3​)+σ3/|K|​​)
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
        h_3x.eval(*beta_3) * van_poly_vkx.eval(*beta_3)
            == ax.eval(*beta_3)
                - (bx.eval(*beta_3)
                    * (*beta_3 * g_3x.eval(*beta_3)
                        + div_mod_val(*sigma_3, Mfp::from(set_k_len as u64))))
    }

    // r(α,β2​)σ3 ​= h2​(β2​) vH​(β2​) + β2​g2​(β2​) +  σ2​​/∣H∣
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
        poly_r.eval(*beta_2) * sigma_3
            == h_2x.eval(*beta_2) * van_poly_vhx.eval(*beta_2)
                + *beta_2 * g_2x.eval(*beta_2)
                + div_mod_val(*sigma_2, Mfp::from(set_h_len as u64))
    }

    // s(β1​)+r(α,β1​)(∑M∈{A,B,C}​ηM​z^M​(β1​))−σ2​z^(β1​) = h1​(β1​)vH​(β1​) + β1​g1​(β1​) + σ1​/∣H∣
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
        println!("3=====================================");
    
        // Print each parameter
        println!("poly_sx: ");
        dsp_poly!(poly_sx);
        
        println!("sum_1: ");
        dsp_poly!(sum_1);
        
        println!("poly_z_hat_x: ");
        dsp_poly!(poly_z_hat_x);
        
        println!("h_1x: ");
        dsp_poly!(h_1x);
        
        println!("g_1x: ");
        dsp_poly!(g_1x);
        
        println!("van_poly_vhx: ");
        dsp_poly!(van_poly_vhx);
        
        println!("beta_1: {beta_1}");
        
        println!("sigma_1: {sigma_1}");

        println!("sigma_2: {sigma_2}");
        
        println!("set_h_len: {}", set_h_len);
        println!("======================================");
        poly_sx.eval(*beta_1) + sum_1.eval(*beta_1) - *sigma_2 * poly_z_hat_x.eval(*beta_1)
            == h_1x.eval(*beta_1) * van_poly_vhx.eval(*beta_1)
                + *beta_1 * g_1x.eval(*beta_1)
                + div_mod_val(*sigma_1, Mfp::from(set_h_len as u64))
    }

    // z^A​(β1​)z^B​(β1​)−z^C​(β1​)=h0​(β1​)vH​(β1​)
    fn check_equation_4(
        poly_ab_c: &Poly,
        poly_h_0: &Poly,
        van_poly_vhx: &Poly,
        beta_1: &Mfp,
    ) -> bool {
        poly_ab_c.eval(*beta_1) == poly_h_0.eval(*beta_1) * van_poly_vhx.eval(*beta_1)
    }

    pub fn check_equation_5(
        val_com_p: Mfp,
        g: Mfp,
        val_y_p: Mfp,
        val_commit_poly_qx: Mfp,
        vk: Mfp,
        z: Mfp,
    ) -> bool {
        e_func(
            div_mod_val(val_com_p, exp_mod(to_bint!(g), to_bint!(val_y_p))),
            g,
            g,
        ) == e_func(
            val_commit_poly_qx,
            div_mod_val(vk, exp_mod(to_bint!(g), to_bint!(z))),
            Mfp::from(g),
        )
    }

    fn gen_poly_ax(
        commitment: &Commitment,
        beta: &[Mfp],
        van_poly_vhx: &Poly,
        eta: &[Mfp],
        poly_pi: &Vec<&Poly>,
    ) -> Poly {
        let poly_sig_a = Poly::from(vec![
            eta[0] * van_poly_vhx.eval(beta[1]) * van_poly_vhx.eval(beta[0]),
        ]) * &commitment.polys_px[2];
        let poly_sig_b = Poly::from(vec![
            eta[1] * van_poly_vhx.eval(beta[1]) * van_poly_vhx.eval(beta[0]),
        ]) * &commitment.polys_px[5];
        let poly_sig_c = Poly::from(vec![
            eta[2] * van_poly_vhx.eval(beta[1]) * van_poly_vhx.eval(beta[0]),
        ]) * &commitment.polys_px[8];
        poly_sig_a * (poly_pi[1] * poly_pi[2])
            + poly_sig_b * (poly_pi[0] * poly_pi[2])
            + poly_sig_c * (poly_pi[0] * poly_pi[1])
    }
}
