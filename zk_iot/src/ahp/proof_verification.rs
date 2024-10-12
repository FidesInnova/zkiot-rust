use ark_ff::Field;
use rustnomial::{Evaluable, FreeSizePolynomial, SizedPolynomial};

use crate::{
    json_file::ClassData,
    math::{
        div_mod, div_mod_val, e_func, exp_mod, func_u, generate_set, kzg, newton_interpolate,
        vanishing_poly, Mfp, Poly, GENERATOR, P,
    },
    to_bint,
    utils::{get_points_set, mat_to_vec},
};

use super::{commitment::Commitment, proof_generation::{AHPData, Polys, ProofGeneration, ProofGenerationJson}};

pub struct Verification {
    pub data: ProofGenerationJson,
}

impl Verification {
    pub fn new(data: ProofGenerationJson ) -> Self {
        Self { data }
    }

    fn get_value(data: &AHPData) -> Mfp {
        match data {
            AHPData::Commit(val) | AHPData::Value(val) => Mfp::from(*val),
            _ => panic!("Unexpected AHPData variant"),
        }
    }

    fn vec_to_poly(data: &AHPData) -> Poly {
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

    pub fn verify(
        &self,
        (ck, vk): (&[Mfp], Mfp),
        class_data: ClassData,
        polys_px: &Vec<Poly>,
        z_vec: Vec<Mfp>,
    ) -> bool {
        let poly_sx = &self.data.get_poly(Polys::Sx as usize);
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
        // let z = hash(poly_sx(22));
        // Randoms:
        let z = Mfp::from(2);
        let alpha = Mfp::from(10);
        let beta = vec![Mfp::from(22), Mfp::from(80), Mfp::from(5)];
        let eta = vec![Mfp::from(2), Mfp::from(30), Mfp::from(100)];
        let t = (class_data.n_i + 1) as usize;

        let set_h_len = class_data.n as usize;
        let set_h = generate_set(set_h_len as u64);
        let set_k_len = class_data.m as usize;

        self.check_1(&polys_px, &beta, &eta, set_h_len, set_k_len)
            && self.check_2(&beta, alpha, set_h_len)
            && self.check_3(z_vec, alpha, &beta, &eta, &set_h, t)
            && self.check_4(&beta, set_h_len)
            && self.check_5((ck, vk), z, Mfp::from(GENERATOR))
    }

    fn check_1(
        &self,
        polys_px: &Vec<Poly>,
        beta: &[Mfp],
        eta: &[Mfp],
        set_h_len: usize,
        set_k_len: usize,
    ) -> bool {
        let van_poly_vkx = Self::vanishing_poly(set_k_len);
        let van_poly_vhx = Self::vanishing_poly(set_h_len);

        let (pi_a, pi_b, pi_c) = ProofGeneration::compute_polys_pi(beta[0], beta[1], &polys_px);
        let polys_pi = vec![&pi_a, &pi_b, &pi_c];


        let poly_a_x = Self::gen_poly_ax(polys_px, beta, &van_poly_vhx, eta, &polys_pi);
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
    
    fn check_2(&self, beta: &[Mfp], alpha: Mfp, set_h_len: usize) -> bool {
        let van_poly_vhx = Self::vanishing_poly(set_h_len);
        let poly_r = func_u(Some(alpha), None, set_h_len);

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

    fn check_3(
        &self,
        x: Vec<Mfp>,
        alpha: Mfp,
        beta: &[Mfp],
        eta: &[Mfp],
        set_h: &Vec<Mfp>,
        t_zero: usize,
    ) -> bool {

        let van_poly_vhx = Self::vanishing_poly(set_h.len());
        let poly_r = func_u(Some(alpha), None, set_h.len());
        let sum_1 = self.gen_poly_sigma(&eta, &poly_r);
        let set_h_1 = &set_h[0..t_zero].to_vec(); // H[>∣x∣]
                                                  // let z_vec = &mat_to_vec(&commitment.matrices.z);
        let points = get_points_set(&x, set_h_1);
        let poly_x_hat = newton_interpolate(&points);
        // Interpolate polynomial w(h) over the subset H[<=∣x∣]
        // Compute the vanishing polynomial for the subset H[<=∣x∣]
        let van_poly_vh1 = vanishing_poly(set_h_1);
        let poly_z_hat_x = &self.data.get_poly(Polys::WHat as usize) * &van_poly_vh1 + poly_x_hat;

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

    fn check_4(&self, beta: &[Mfp], set_h_len: usize) -> bool {
        let van_poly_vhx = Self::vanishing_poly(set_h_len);
        let poly_ab_c = &self.data.get_poly(Polys::ZHatA as usize) * &self.data.get_poly(Polys::ZHatB as usize)
            - &self.data.get_poly(Polys::ZHatC as usize);
        let poly_h_0 = div_mod(&poly_ab_c, &van_poly_vhx).0;
        Self::check_equation_4(&poly_ab_c, &poly_h_0, &van_poly_vhx, &beta[0])
    }

    fn check_5(&self, (ck, vk): (&[Mfp], Mfp), z: Mfp, g: Mfp) -> bool {
        // TODO: All random (1..P)
        let eta_values = vec![
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
            .map(|(i, &eta)| Poly::from(vec![eta]) * &self.data.get_poly(i).clone())
            .fold(Poly::zero(), |acc, poly| acc + poly);

        let val_y_p = poly_px.eval(z);
        let val_commit_poly_px = kzg::commit(&poly_px, ck);
        let mut poly_px_add = poly_px;
        poly_px_add.add_term(-val_y_p, 0);
        let poly_x_z = Poly::from(vec![Mfp::ONE, Mfp::from(-z)]);
        let poly_qx = div_mod(&poly_px_add, &poly_x_z).0;
        let val_commit_poly_qx = kzg::commit(&poly_qx, &ck);
        Self::check_equation_5(val_commit_poly_px, g, val_y_p, val_commit_poly_qx, vk, z)
    }

    
    #[inline]
    fn gen_poly_sigma(&self, eta: &[Mfp], poly_r: &Poly) -> Poly {
        let sigma_eta_z_x = Poly::new(vec![eta[0]]) * &self.data.get_poly(Polys::ZHatA as usize)
            + Poly::new(vec![eta[1]]) * &self.data.get_poly(Polys::ZHatB as usize)
            + Poly::new(vec![eta[2]]) * &self.data.get_poly(Polys::ZHatC as usize);
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
        println!("eq11: {}", h_3x.eval(*beta_3) * van_poly_vkx.eval(*beta_3));
        println!(
            "eq12: {}",
            ax.eval(*beta_3)
                - (bx.eval(*beta_3)
                    * (*beta_3 * g_3x.eval(*beta_3)
                        + div_mod_val(*sigma_3, Mfp::from(set_k_len as u64))))
        );
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
        println!("eq21: {}", poly_r.eval(*beta_2) * sigma_3);
        println!(
            "eq22: {}",
            h_2x.eval(*beta_2) * van_poly_vhx.eval(*beta_2)
                + *beta_2 * g_2x.eval(*beta_2)
                + div_mod_val(*sigma_2, Mfp::from(set_h_len as u64))
        );

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
        println!(
            "eq31: {}",
            poly_sx.eval(*beta_1) + sum_1.eval(*beta_1) - *sigma_2 * poly_z_hat_x.eval(*beta_1)
        );
        println!(
            "eq32: {}",
            h_1x.eval(*beta_1) * van_poly_vhx.eval(*beta_1)
                + *beta_1 * g_1x.eval(*beta_1)
                + div_mod_val(*sigma_1, Mfp::from(set_h_len as u64))
        );
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
        println!("eq41: {}", poly_ab_c.eval(*beta_1));
        println!(
            "eq42: {}",
            poly_h_0.eval(*beta_1) * van_poly_vhx.eval(*beta_1)
        );
        poly_ab_c.eval(*beta_1) == poly_h_0.eval(*beta_1) * van_poly_vhx.eval(*beta_1)
    }

    pub fn check_equation_5(
        val_commit_poly_px: Mfp,
        g: Mfp,
        val_y_p: Mfp,
        val_commit_poly_qx: Mfp,
        vk: Mfp,
        z: Mfp,
    ) -> bool {
        println!("val_commit_px: {val_commit_poly_px}, val_y_p: {val_y_p}, vk: {vk}, val_commit_poly_qx: {val_commit_poly_qx}");

        let e_1 = e_func(
            val_commit_poly_px - Mfp::from(to_bint!(g) * to_bint!(val_y_p)),
            g,
            g,
        );
        println!("eq51: {}", e_1);

        let e_2 = e_func(
            val_commit_poly_qx,
            vk - Mfp::from(to_bint!(g) * to_bint!(z)),
            Mfp::from(g),
        );
        println!("eq52: {}", e_2);
        e_1 == e_2
    }

    fn gen_poly_ax(
        polys_px: &Vec<Poly>,
        beta: &[Mfp],
        van_poly_vhx: &Poly,
        eta: &[Mfp],
        poly_pi: &Vec<&Poly>,
    ) -> Poly {
        let val_vhx_beta_1 = van_poly_vhx.eval(beta[0]);
        println!("val_vhx_beta_1: {val_vhx_beta_1}");
        let val_vhx_beta_2 = van_poly_vhx.eval(beta[1]);
        println!("val_vhx_beta_2: {val_vhx_beta_2}");

        let poly_sig_a = Poly::from(vec![eta[0] * val_vhx_beta_2 * val_vhx_beta_1]) * &polys_px[2];
        let poly_sig_b = Poly::from(vec![eta[1] * val_vhx_beta_2 * val_vhx_beta_1]) * &polys_px[5];
        let poly_sig_c = Poly::from(vec![eta[2] * val_vhx_beta_2 * val_vhx_beta_1]) * &polys_px[8];
        poly_sig_a * (poly_pi[1] * poly_pi[2])
            + poly_sig_b * (poly_pi[0] * poly_pi[2])
            + poly_sig_c * (poly_pi[0] * poly_pi[1])
    }
}
