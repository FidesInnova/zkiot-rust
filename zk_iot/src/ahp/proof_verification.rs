use ark_ff::Field;
use rustnomial::{Evaluable, FreeSizePolynomial};

use crate::{
    dsp_poly,
    math::{div_mod, div_mod_val, e_func, exp_mod, func_u, Mfp, Poly},
    to_bint,
};

use super::proof_generation::AHPData;

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

    pub fn verify(&self, set_h_len: usize, set_k_len: usize, vk: Mfp) -> bool {
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

        // Randoms:
        let alpha = Mfp::from(10);
        let eta = vec![Mfp::from(2), Mfp::from(30), Mfp::from(100)];
        let beta = vec![Mfp::from(22), Mfp::from(80), Mfp::from(5)];
        // Polynomials
        let van_poly_vkx = Self::vanishing_poly(set_k_len);
        let van_poly_vhx = Self::vanishing_poly(set_h_len);
        let poly_r = func_u(Some(alpha), None, set_h_len);
        // let poly_a_x = &Self::get_poly(&self.data[26]) * &van_poly_vkx + Self::get_poly(&self.data[25]);
        // let poly_b_x = &Self::get_poly(&self.data[26]) * &van_poly_vkx + Self::get_poly(&self.data[21]);
        let sum_1 = Self::gen_poly_sigma(&eta, &self.data, &poly_r);
        let poly_ab_c = &Self::get_poly(&self.data[14]) * &Self::get_poly(&self.data[15]) - &Self::get_poly(&self.data[16]);
        let poly_h_0 = div_mod(&poly_ab_c, &van_poly_vhx).0;

        // let poly_z_hat_x = &Self::get_poly(self.data[13]) * &van_poly_vh1 + poly_x_hat;

        // Self::check_equation_1(
        //     &Self::get_poly(&self.data[26]),
        //     &Self::get_poly(&self.data[25]),
        //     &van_poly_vkx,
        //     &poly_a_x,
        //     &poly_b_x,
        //     &beta_3,
        //     &Self::get_value(&self.data[24]),
        //     set_k_len,
        // ) &&
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
        // &&  Self::check_equation_3(
        //     &poly_sx,
        //     &sum_1,
        //     &poly_z_hat_x,
        //     &Self::get_poly(&self.data[20]),
        //     &Self::get_poly(&self.data[19]),
        //     &van_poly_vhx,
        //     &beta_1,
        //     &Self::get_value(&self.data[12]),
        //     &Self::get_value(&self.data[21]),
        //     set_h_len,
        // )
        && Self::check_equation_4(&poly_ab_c, &poly_h_0, &van_poly_vhx, &beta[0])
        // && Self::check_equation_5(val_com_p, Mfp::from(g), val_y_p, val_commit_poly_qx, vk, z)
    }

    #[inline]
    fn gen_poly_sigma(eta: &Vec<Mfp>, data: &[AHPData], poly_r: &Poly) -> Poly {
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
        println!("eq1 =======================================");
        // Print all Poly input arguments using dsp_poly!
        println!("h_3x in");
        dsp_poly!(h_3x);

        println!("g_3x in");
        dsp_poly!(g_3x);

        println!("van_poly_vkx in");
        dsp_poly!(van_poly_vkx);

        println!("ax in");
        dsp_poly!(ax);

        println!("bx in");
        dsp_poly!(bx);

        // Print Mfp input arguments
        println!("beta_3: {:?}", beta_3); // Assuming Mfp implements Debug
        println!("sigma_3: {:?}", sigma_3); // Assuming Mfp implements Debug

        // Print the length
        println!("set_k_len: {}", set_k_len);

        println!("===========================================");

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
        println!("eq2 =======================================");

        // Print all inputs
        println!("poly_r in: ");
        dsp_poly!(poly_r);

        println!("h_2x in: ");
        dsp_poly!(h_2x);

        println!("g_2x in: ");
        dsp_poly!(g_2x);

        println!("van_poly_vhx in: ");
        dsp_poly!(van_poly_vhx);

        println!("beta_2 in: ");
        println!("{:?}", beta_2);

        println!("sigma_2 in: ");
        println!("{:?}", sigma_2);

        println!("sigma_3 in: ");
        println!("{:?}", sigma_3);

        println!("set_h_len in: {}", set_h_len);
        println!("===========================================");

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
}
