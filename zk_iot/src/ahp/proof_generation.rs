
use crate::ahp::commitmnet::Commitment;
use crate::dsp_poly;
use crate::dsp_vec;
use crate::json_file::write_term;
use crate::math::*;
use crate::setup::Setup;
use crate::to_bint;
use crate::utils::*;
use anyhow::Result;
use ark_ff::Field;
use rustnomial::Evaluable;
use rustnomial::FreeSizePolynomial;
use rustnomial::SizedPolynomial;

#[derive(Debug)]
pub enum AHPData {
    Commit(u64),
    Value(u64),
    Polynomial(Vec<u64>),
}
pub struct ProofGeneration {}

impl ProofGeneration {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_proof(
        &self,
        commitmnet: &Commitment,
        commitment_key: &Vec<Mfp>,
        generator: u64,
    ) -> Box<[AHPData]> {
        // Convert matrices to vectors and retrieve corresponding points
        let mut points_za =
            get_points_set(&mat_to_vec(&commitmnet.get_matrix_az()), &commitmnet.set_h);
        let mut points_zb =
            get_points_set(&mat_to_vec(&commitmnet.get_matrix_bz()), &commitmnet.set_h);
        let mut points_zc =
            get_points_set(&mat_to_vec(&commitmnet.get_matrix_cz()), &commitmnet.set_h);
            
        // From wiki: [https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/3-proof-generation-phase#id-3-5-2-ahp-proof]
        // Random inertation for za:
        points_za.push((Mfp::from(150), Mfp::from(5)));
        points_za.push((Mfp::from(80), Mfp::from(47)));
        // Random inertation for zb:
        points_zb.push((Mfp::from(150), Mfp::from(15)));
        points_zb.push((Mfp::from(80), Mfp::from(170)));
        // Random inertation for zc:
        points_zc.push((Mfp::from(150), Mfp::from(1)));
        points_zc.push((Mfp::from(80), Mfp::from(100)));

        // Interpolate polynomials for za, zb, and zc
        let poly_z_hat_a = lagrange_interpolate(&points_za);
        println!("^za(x):");
        dsp_poly!(poly_z_hat_a);
        let poly_z_hat_b = lagrange_interpolate(&points_zb);
        println!("^zb(x):");
        dsp_poly!(poly_z_hat_b);
        let poly_z_hat_c = lagrange_interpolate(&points_zc);
        println!("^zc(x):");
        dsp_poly!(poly_z_hat_c);

        // Split set_h into two subsets based on index t
        let set_h_1 = &commitmnet.set_h[0..commitmnet.numebr_t_zero].to_vec(); // H[>∣x∣]
        let set_h_2 = &commitmnet.set_h[commitmnet.numebr_t_zero..].to_vec(); // H[<=∣x∣]

        // Interpolate polynomial for x^(h) over the subset H[>∣x∣]
        let z_vec = &mat_to_vec(&commitmnet.matrices.z);
        let points = get_points_set(&z_vec[0..commitmnet.numebr_t_zero], set_h_1);
        let poly_x_hat = lagrange_interpolate(&points);

        // Interpolate polynomial w(h) over the subset H[<=∣x∣]
        let z_vec = &mat_to_vec(&commitmnet.matrices.z);
        let points = get_points_set(&z_vec[commitmnet.numebr_t_zero..], set_h_2);
        let wh = lagrange_interpolate(&points);

        // Compute the vanishing polynomial for the subset H[<=∣x∣]
        let van_poly_vh1 = vanishing_poly(set_h_1);

        println!("van_poly_vh1: ");
        dsp_poly!(van_poly_vh1);

        let mut points_w = vec![];
        for i in set_h_2 {
            // Compute the adjusted polynomial wˉ(h) for each element in the subset

            let w_bar_h = (wh.eval(*i) - poly_x_hat.eval(*i))
                * exp_mod(to_bint!(van_poly_vh1.eval(*i)), P - 2);
            points_w.push((*i, w_bar_h));
        }

        // TODO:
        // Uncomment this line to insert random points for wˉ(h) from the set
        // push_random_points(&mut points_w, b, &vec_to_set(&commitmnet.set_h));
        // From wiki: [https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/3-proof-generation-phase#id-3-5-2-ahp-proof]
        points_w.push((Mfp::from(150), Mfp::from(42)));
        points_w.push((Mfp::from(80), Mfp::from(180)));

        // Interpolate polynomial for wˉ(h) based on the points_w
        let poly_w_hat = lagrange_interpolate(&points_w);

        println!("w_hat:"); // Output the interpolated polynomial for wˉ(h)
        dsp_poly!(poly_w_hat);

        // h_zero
        let van_poly_vhx = vanishing_poly(&commitmnet.set_h);
        println!("van_poly_vhx: ");
        dsp_poly!(van_poly_vhx);

        let poly_ab_c = &poly_z_hat_a * &poly_z_hat_b - &poly_z_hat_c;
        let poly_h_0 = (&poly_ab_c).div_mod(&van_poly_vhx).0;

        println!("h0(x):");
        dsp_poly!(poly_h_0);

        // Generate a random polynomial (currently hardcoded for demonstration)
        let poly_sx = [5, 0, 101, 17, 0, 1, 20, 0, 0, 3, 115];
        let poly_sx = poly_sx.iter().map(|v| Mfp::from(*v)).collect::<Vec<Mfp>>();
        let poly_sx = Poly::from(poly_sx);

        println!("poly_sx");
        dsp_poly!(poly_sx);

        // Compute sigma by evaluating the polynomial at points in set_h
        let sigma_1 = commitmnet
            .set_h
            .iter()
            .fold(Mfp::ZERO, |acc, &v| acc + poly_sx.eval(v));
        println!("sigma_1 :\t{}", sigma_1);

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
        let poly_r = func_u(Some(alpha), None, commitmnet.set_h.len());
        println!("r:");
        dsp_poly!(poly_r);

        println!("r(alpha_2 , x) ∑_m [η_M z^_M(x)]:");
        dsp_poly!((&poly_r * &sigma_eta_z_x));

        // r(α,x) * ∑_m [η_M ​z^M​(x)]
        let sum_1 = &poly_r * sigma_eta_z_x;

        // Compute polynomial for Z^(x)
        let poly_z_hat_x = &poly_w_hat * &van_poly_vh1 + poly_x_hat;
        println!("z_hat: ");
        dsp_poly!(poly_z_hat_x);

        // ∑ r(alpha_2=10, k) * A^(k,x)
        let r_a_kx = sigma_rkx_mkx(
            &commitmnet.set_h,
            alpha,
            &commitmnet.points_px[0],
            &commitmnet.points_px[1],
            &commitmnet.points_px[2],
        );

        println!("Poly ∑ r(alpha_2=10, k) * A^(k,x): A_h");
        dsp_poly!(r_a_kx);

        // ∑ r(alpha_2=10, k) * B^(k,x)
        let r_b_kx = sigma_rkx_mkx(
            &commitmnet.set_h,
            alpha,
            &commitmnet.points_px[3],
            &commitmnet.points_px[4],
            &commitmnet.points_px[5],
        );
        println!("Poly ∑ r(alpha_2=10, k) * B^(k,x): ");
        dsp_poly!(r_b_kx);

        // ∑ r(alpha_2=10, k) * C^(k,x)
        let r_c_kx = sigma_rkx_mkx(
            &commitmnet.set_h,
            alpha,
            &commitmnet.points_px[6],
            &commitmnet.points_px[7],
            &commitmnet.points_px[8],
        );
        println!("Poly ∑ r(alpha_2=10, k) * C^(k,x): ");
        dsp_poly!(r_c_kx);

        // ∑_m [η_M r_M(α,x)] * z^(x)
        let sum_2 = Poly::new(vec![eta_a]) * &r_a_kx
            + Poly::new(vec![eta_b]) * &r_b_kx
            + Poly::new(vec![eta_c]) * &r_c_kx;
        let sum_2 = sum_2 * &poly_z_hat_x;

        // Sum Check Protocol Formula:
        // s(x) + r(α,x) * ∑_m [η_M ​z^M​(x)] - ∑_m [η_M r_M(α,x)] * z^(x)
        let poly_scp = poly_sx.clone() + sum_1.clone() - &sum_2;

        println!("scp: ");
        dsp_poly!(poly_scp);

        let div_res = poly_scp.div_mod(&van_poly_vhx);
        let h_1x = div_res.0;
        println!("Poly h_1x: ");
        dsp_poly!(h_1x);

        let g_1x = div_res.1.div_mod(&Poly::new(vec![Mfp::ONE, Mfp::ZERO])).0;
        println!("Poly g_1x:");
        dsp_poly!(g_1x);

        // TODO: Random F - H
        // let beta_1 = gen_rand_not_in_set(&vec_to_set(&commitmnet.set_h));
        // let beta_1 = Mfp::from(sha2_hash(&poly_sx.eval(Mfp::from(9)).to_string()));
        let beta_1 = Mfp::from(22);

        // ∑ r(alpha_2=10, k) * A^(x,k)
        let r_a_xk = m_xk(
            &beta_1,
            &commitmnet.points_px[0],
            &commitmnet.points_px[1],
            &commitmnet.points_px[2],
            commitmnet.set_h.len(),
        );
        println!("Poly ∑ r(alpha_2=10, k) * A^(x,k): ");
        dsp_poly!(r_a_xk);

        // ∑ r(alpha_2=10, k) * B^(x,k)
        let r_b_xk = m_xk(
            &beta_1,
            &commitmnet.points_px[3],
            &commitmnet.points_px[4],
            &commitmnet.points_px[5],
            commitmnet.set_h.len(),
        );
        println!("Poly ∑ r(alpha_2=10, k) * B^(x,k): ");
        dsp_poly!(r_b_xk);

        // ∑ r(alpha_2=10, k) * C^(x,k)
        let r_c_xk = m_xk(
            &beta_1,
            &commitmnet.points_px[6],
            &commitmnet.points_px[7],
            &commitmnet.points_px[8],
            commitmnet.set_h.len(),
        );
        println!("Poly ∑ r(alpha_2=10, k) * C^(x,k): ");
        dsp_poly!(r_c_xk);

        // sigma_2
        let sigma_2 =
            eta_a * r_a_kx.eval(beta_1) + eta_b * r_b_kx.eval(beta_1) + eta_c * r_c_kx.eval(beta_1);
        println!("sigma_2: {}", sigma_2);

        // r(alpha_2=10, x) ∑_m [​η_M ​M^(x,β1​)]
        let poly_sigma_2 = Poly::new(vec![eta_a]) * r_a_xk
            + Poly::new(vec![eta_b]) * r_b_xk
            + Poly::new(vec![eta_c]) * r_c_xk;
        let poly_sigma_2 = &poly_r * poly_sigma_2;

        println!("r(alpha_2=10, x) * ∑_m [η_M M^(x, β1)]: ");
        dsp_poly!(poly_sigma_2);

        let div_res = poly_sigma_2.div_mod(&van_poly_vhx);
        let h_2x = div_res.0;
        println!("Poly h_2x: ");
        dsp_poly!(h_2x);

        let g_2x = div_res.1.div_mod(&Poly::new(vec![Mfp::ONE, Mfp::ZERO])).0;
        println!("Poly g_2x:");
        dsp_poly!(g_2x);

        // sigma_3
        let mut sigma_3 = Mfp::ZERO;

        // TODO: Random F - H
        // let beta_2 = gen_rand_not_in_set(&vec_to_set(&commitmnet.set_h));
        // let beta_2 = Mfp::from(sha2_hash(&poly_sx.eval(Mfp::from(10)).to_string()));
        let beta_2 = Mfp::from(80);

        // f_3x
        let mut points_f_3: Vec<Point> = vec![];
        for k in commitmnet.set_k.iter() {
            let sig_a = sigma_m(
                &van_poly_vhx,
                &eta_a,
                &beta_1,
                &beta_2,
                k,
                &[
                    &commitmnet.polys_px[0],
                    &commitmnet.polys_px[1],
                    &commitmnet.polys_px[2],
                ],
            );
            let sig_b = sigma_m(
                &van_poly_vhx,
                &eta_b,
                &beta_1,
                &beta_2,
                k,
                &[
                    &commitmnet.polys_px[3],
                    &commitmnet.polys_px[4],
                    &commitmnet.polys_px[5],
                ],
            );
            let sig_c = sigma_m(
                &van_poly_vhx,
                &eta_c,
                &beta_1,
                &beta_2,
                k,
                &[
                    &commitmnet.polys_px[6],
                    &commitmnet.polys_px[7],
                    &commitmnet.polys_px[8],
                ],
            );

            let sum = sig_a + sig_b + sig_c;
            sigma_3 += sum;
            points_f_3.push((*k, sum));
        }
        println!("sigma_3: {}", sigma_3);

        
        let poly_pi_a = (Poly::from(vec![beta_2]) - &commitmnet.polys_px[0])
            * (Poly::from(vec![beta_1]) - &commitmnet.polys_px[1]);
        let poly_pi_b = (Poly::from(vec![beta_2]) - &commitmnet.polys_px[3])
            * (Poly::from(vec![beta_1]) - &commitmnet.polys_px[4]);
        let poly_pi_c = (Poly::from(vec![beta_2]) - &commitmnet.polys_px[6])
            * (Poly::from(vec![beta_1]) - &commitmnet.polys_px[7]);
        let polys_pi = vec![&poly_pi_a, &poly_pi_b, &poly_pi_c]; 
        dsp_poly!(poly_pi_a);
        dsp_poly!(poly_pi_b);
        dsp_poly!(poly_pi_b);


        let poly_a_x = Self::gen_poly_ax(commitmnet, vec![beta_1, beta_2], &van_poly_vhx, vec![eta_a, eta_b, eta_c], &polys_pi);

        println!("poly_a_x: {}", poly_a_x.eval(Mfp::from(5)));
        dsp_poly!(poly_a_x);

        // b(x)
        let poly_b_x = polys_pi[0] * polys_pi[1] * polys_pi[2];

        println!("poly_b_x: {}", poly_b_x.eval(Mfp::from(5)));
        dsp_poly!(poly_b_x);

        let van_poly_vkx = vanishing_poly(&commitmnet.set_k);

        println!("van_poly_vkx");
        dsp_poly!(van_poly_vkx);

        let poly_f_3x = lagrange_interpolate(&points_f_3);

        let sigma_3_set_k = Mfp::from(sigma_3 * (exp_mod(commitmnet.set_k.len() as u64, P - 2)));
        println!("sigma_3_set_k {}", sigma_3_set_k);

        let poly_f_3x = poly_f_3x - Poly::from(vec![sigma_3_set_k]);

        println!("poly_f_3x");
        dsp_poly!(poly_f_3x);

        let g_3x = poly_f_3x.div_mod(&Poly::from(vec![Mfp::ONE, Mfp::ZERO])).0;

        println!("g_3x");
        dsp_poly!(g_3x);

        let h_3x = (poly_a_x.clone()
            - (&poly_b_x * (poly_f_3x.clone() + Poly::from(vec![sigma_3_set_k]))))
            .div_mod(&van_poly_vkx)
            .0;

        println!("h_3x");
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
            .map(|(i, &eta)| Poly::from(vec![eta]) * polys_proof[i].clone())
            .fold(Poly::zero(), |acc, poly| acc + poly);

        println!("poly_px: ");
        dsp_poly!(poly_px);

        // hash(poly_sx(22));
        let z = Mfp::from(2);
        let val_y_p = poly_px.eval(z);

        println!("val_y_p {}", val_y_p);

        let mut poly_px_add = poly_px;
        poly_px_add.add_term(-val_y_p, 0);
        let poly_x_z = Poly::from(vec![Mfp::ONE, Mfp::from(-z)]);

        let poly_qx = div_mod(&poly_px_add, &poly_x_z).0;

        println!("poly_qx");
        dsp_poly!(poly_qx);

        let val_commit_poly_qx = kzg::commit(&poly_qx, commitment_key, generator);
        println!("val_commit_qx: {}", val_commit_poly_qx);

        let sigma = [
            sigma_1,
            sigma_2,
            sigma_3,
        ];

        let commit_x = compute_all_commitment(&polys_proof, commitment_key, generator);
        println!("commit_x: {}", dsp_vec!(commit_x));

        Self::create_proof(&polys_proof, &sigma, &commit_x, val_y_p, val_commit_poly_qx)
    }

    fn create_proof(polys_proof: &[Poly], sigma: &[Mfp], commit_x: &[Mfp], val_y_p: Mfp, val_commit_poly_qx: Mfp) -> Box<[AHPData]> {
        let pi_ahp = [
            AHPData::Commit(to_bint!(commit_x[0])),             // [0]: COM1AHP
            AHPData::Commit(to_bint!(commit_x[1])),             // [1]: COM2AHP
            AHPData::Commit(to_bint!(commit_x[2])),             // [2]: COM3AHP
            AHPData::Commit(to_bint!(commit_x[3])),             // [3]: COM4AHP
            AHPData::Commit(to_bint!(commit_x[4])),             // [4]: COM5AHP
            AHPData::Commit(to_bint!(commit_x[5])),             // [5]: COM6AHP
            AHPData::Commit(to_bint!(commit_x[6])),             // [6]: COM7AHP
            AHPData::Commit(to_bint!(commit_x[7])),             // [7]: COM8AHP
            AHPData::Commit(to_bint!(commit_x[8])),             // [8]: COM9AHP
            AHPData::Commit(to_bint!(commit_x[9])),             // [9]: COM10AHP
            AHPData::Commit(to_bint!(commit_x[10])),            // [10]: COM11AHP
            AHPData::Commit(to_bint!(commit_x[11])),            // [11]: COM12AHP
            AHPData::Value(to_bint!(sigma[0])),                 // [12]: P1AHP: sigma_1
            AHPData::Polynomial(write_term(&polys_proof[0])),   // [13]: P2AHP: w^x
            AHPData::Polynomial(write_term(&polys_proof[1])),   // [14]: P3AHP: z^a
            AHPData::Polynomial(write_term(&polys_proof[2])),   // [15]: P4AHP: z^b
            AHPData::Polynomial(write_term(&polys_proof[3])),   // [16]: P5AHP: z^c
            AHPData::Polynomial(write_term(&polys_proof[4])),   // [17]: P6AHP: h_0
            AHPData::Polynomial(write_term(&polys_proof[5])),   // [18]: P7AHP: sx
            AHPData::Polynomial(write_term(&polys_proof[6])),   // [19]: P8AHP: g_1
            AHPData::Polynomial(write_term(&polys_proof[7])),   // [20]: P9AHP: h_1
            AHPData::Value(to_bint!(sigma[1])),                 // [21]: P10AHP: sigma_2
            AHPData::Polynomial(write_term(&polys_proof[8])),   // [22]: P11AHP: g_2
            AHPData::Polynomial(write_term(&polys_proof[9])),   // [23]: P12AHP: h_2
            AHPData::Value(to_bint!(sigma[2])),                 // [24]: P13AHP: sigma_3
            AHPData::Polynomial(write_term(&polys_proof[10])),  // [25]: P14AHP: g_3
            AHPData::Polynomial(write_term(&polys_proof[11])),  // [26]: P15AHP: h_3
            AHPData::Value(to_bint!(val_y_p)),                  // [27]: P16AHP: y'
            AHPData::Value(to_bint!(val_commit_poly_qx)),       // [28]: P17AHP: val_commit_poly_qx
        ];

        Box::new(pi_ahp)
    }

    fn gen_poly_ax(commitmnet: &Commitment, beta: Vec<Mfp>, van_poly_vhx: &Poly, eta: Vec<Mfp>, poly_pi: &Vec<&Poly>) -> Poly {
        let poly_sig_a = Poly::from(vec![
            eta[0] * van_poly_vhx.eval(beta[1]) * van_poly_vhx.eval(beta[0]),
        ]) * &commitmnet.polys_px[2];
        let poly_sig_b = Poly::from(vec![
            eta[1] * van_poly_vhx.eval(beta[1]) * van_poly_vhx.eval(beta[0]),
        ]) * &commitmnet.polys_px[5];
        let poly_sig_c = Poly::from(vec![
            eta[2] * van_poly_vhx.eval(beta[1]) * van_poly_vhx.eval(beta[0]),
        ]) * &commitmnet.polys_px[8];

        dsp_poly!(poly_sig_a);
        dsp_poly!(poly_sig_b);
        dsp_poly!(poly_sig_c);

        poly_sig_a * (poly_pi[1] * poly_pi[2])
        + poly_sig_b * (poly_pi[0] * poly_pi[2])
        + poly_sig_c * (poly_pi[0] * poly_pi[1])
    }

    pub fn store(&self, path: &str) -> Result<()> {
        todo!()
    }

    pub fn restore(path: &str) -> Result<Self> {
        todo!()
    }
}
