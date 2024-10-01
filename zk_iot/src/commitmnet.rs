pub mod ahp {
    use std::collections::HashMap;

    use crate::{
        dsp_mat, dsp_poly, json_file::store_commit_json, math::*, setup::*, to_bint, utils::*
    };
    use anyhow::Result;
    use ark_ff::{Field, PrimeField};
    use nalgebra::DMatrix;
    use rustnomial::Evaluable;

    pub struct Commitment {
        pub set_h_len: u64,
        pub set_k_len: u64,
        pub set_h: Vec<Mfp>,
        pub set_k: Vec<Mfp>,
        pub numebr_t_zero: usize,
        pub matrices: Matrices,
        pub polynomials: HashMap<String, Poly>,
    }

    impl Commitment {
        // Constructor method Generate sets and Initilize matrices
        pub fn new(setup: &Setup) -> Result<Self> {
            let set_h_len: u64 = (setup.number_gate + setup.number_input + 1).try_into()?;
            let numebr_t_zero: u64 = (setup.number_input + 1).try_into()?; // Number of rows (|x| = self.numebr_t_zero, where self.numebr_t_zero = ni + 1)
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
                polynomials: HashMap::new(),
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

        pub fn commit_o(&self, long_const_val: u64, generator: u64) -> Vec<Mfp> {
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

            commit(&o_i, long_const_val, generator) // Generate the commitment
        }

        pub fn commit(&self) {
            // Compute matrix multiplications for A, B, and C with z_poly
            let az: DMatrix<Mfp> = &self.matrices.a * &self.matrices.z;
            let bz = &self.matrices.b * &self.matrices.z;
            let cz = &self.matrices.c * &self.matrices.z;

            // Display the results of matrix multiplications
            println!("Matrix Az:");
            dsp_mat!(az);
            println!("Matrix Bz");
            dsp_mat!(bz);
            println!("Matrix Cz");
            dsp_mat!(cz);

            // Convert matrices to vectors and retrieve corresponding points
            let mut points_za = get_points_set(&mat_to_vec(&az), &self.set_h);
            let mut points_zb = get_points_set(&mat_to_vec(&bz), &self.set_h);
            let mut points_zc = get_points_set(&mat_to_vec(&cz), &self.set_h);

            // Uncomment and adjust the line below to push random points
            // TODO: Define a random value b within the range F(0..P-n) and ensure 0 < b <= P - n
            let b = 2;
            push_random_points(&mut points_za, b, &vec_to_set(&self.set_h));
            push_random_points(&mut points_zb, b, &vec_to_set(&self.set_h));
            push_random_points(&mut points_zc, b, &vec_to_set(&self.set_h));

            // Interpolate polynomials for za, zb, and zc
            let poly_z_hat_a = lagrange_interpolate(&points_za);
            println!("^za(x):");
            dsp_poly!(&poly_z_hat_a);
            let poly_z_hat_b = lagrange_interpolate(&points_zb);
            println!("^zb(x):");
            dsp_poly!(&poly_z_hat_b);
            let poly_z_hat_c = lagrange_interpolate(&points_zc);
            println!("^zc(x):");
            dsp_poly!(&poly_z_hat_c);

            // Split set_h into two subsets based on index self.numebr_t_zero
            let set_h_1 = &self.set_h[0..self.numebr_t_zero].to_vec(); // H[>∣x∣]
            let set_h_2 = &self.set_h[self.numebr_t_zero..].to_vec(); // H[<=∣x∣]

            // Interpolate polynomial for x^(h) over the subset H[>∣x∣]
            let z_vec = &mat_to_vec(&self.matrices.z);
            let points = get_points_set(&z_vec[0..self.numebr_t_zero], set_h_1);
            let poly_x_hat = lagrange_interpolate(&points);

            // Interpolate polynomial w(h) over the subset H[<=∣x∣]
            let z_vec = &mat_to_vec(&self.matrices.z);
            let points = get_points_set(&z_vec[self.numebr_t_zero..], set_h_2);
            let wh = lagrange_interpolate(&points);

            // Compute the vanishing polynomial for the subset H[<=∣x∣]
            let van_poly_vh1 = vanishing_poly(set_h_1);

            println!("van_poly_vh1: ");
            dsp_poly!(&van_poly_vh1);

            let mut points_w = vec![];
            for i in set_h_2 {
                // Compute the adjusted polynomial wˉ(h) for each element in the subset
                let w_bar_h =
                    (wh.eval(*i) - poly_x_hat.eval(*i)) * van_poly_vh1.eval(*i).inverse().unwrap();
                points_w.push((*i, w_bar_h));
            }

            // Uncomment these lines to insert random points for wˉ(h) from the set
            push_random_points(&mut points_w, b, &vec_to_set(&self.set_h));

            // Interpolate polynomial for wˉ(h) based on the points_w
            let poly_w_hat = lagrange_interpolate(&points_w);

            println!("w_hat:"); // Output the interpolated polynomial for wˉ(h)
            dsp_poly!(&poly_w_hat);

            // h_zero
            let van_poly_vhx = vanishing_poly(&self.set_h);
            println!("van_poly_vhx: ");
            dsp_poly!(&van_poly_vhx);

            let poly_ab_c = &poly_z_hat_a * &poly_z_hat_b - &poly_z_hat_c;
            let poly_h_0 = (&poly_ab_c).div_mod(&van_poly_vhx).0;

            println!("h0(x):");
            dsp_poly!(&poly_h_0);

            // Generate a random polynomial (currently hardcoded for demonstration)
            let poly_sx = [5, 0, 101, 17, 0, 1, 20, 0, 0, 3, 115];
            let poly_sx = poly_sx.iter().map(|v| Mfp::from(*v)).collect::<Vec<Mfp>>();
            let poly_sx = Poly::new(poly_sx);

            println!("poly_sx");
            dsp_poly!(&poly_sx);

            // Compute sigma by evaluating the polynomial at points in set_h
            let sigma_1 = self
                .set_h
                .iter()
                .fold(Mfp::ZERO, |acc, &v| acc + poly_sx.eval(v));
            println!("sigma_1 :{}", sigma_1);

            let alpha = Mfp::from(sha2_hash(
                &to_bint!(poly_sx.eval(Mfp::from(0)) + poly_sx.eval(Mfp::from(1)) + Mfp::from(1))
                    .to_string(),
            ));
            let eta_a = Mfp::from(sha2_hash(
                &(poly_sx.eval(Mfp::from(2)) + poly_sx.eval(Mfp::from(3)) + Mfp::from(2))
                    .to_string(),
            ));
            let eta_b = Mfp::from(sha2_hash(
                &(poly_sx.eval(Mfp::from(4)) + poly_sx.eval(Mfp::from(5)) + Mfp::from(3))
                    .to_string(),
            ));
            let eta_c = Mfp::from(sha2_hash(
                &(poly_sx.eval(Mfp::from(6)) + poly_sx.eval(Mfp::from(7)) + Mfp::from(4))
                    .to_string(),
            ));

            // Compute polynomial for ∑ ηz(x)
            let sigma_eta_z_x = Poly::new(vec![eta_a]) * &poly_z_hat_a
                + Poly::new(vec![eta_b]) * &poly_z_hat_b
                + Poly::new(vec![eta_c]) * &poly_z_hat_c;

            // Compute polynomial for r(α,x) ∑ ηM(z^M(x))
            let poly_r = func_u(Some(alpha), None, self.set_k_len.try_into().unwrap());
            println!("r:");
            dsp_poly!(&poly_r);

            println!("r(alpha , x) ∑_m [η_M z^_M(x)]:");
            dsp_poly!(&(&poly_r * &sigma_eta_z_x));

            // r(α,x) * ∑_m [η_M ​z^M​(x)]
            let sum_1 = &poly_r * sigma_eta_z_x;

            // Compute polynomial for Z^(x)
            let poly_z_hat_x = &poly_w_hat * &van_poly_vh1 + poly_x_hat;
            println!("z_hat: ");
            dsp_poly!(&poly_z_hat_x);

            // Matrix A:
            let points_row_p_a = get_matrix_point_row(&self.matrices.a, &self.set_h, &self.set_k);
            let points_col_p_a = get_matrix_point_col(&self.matrices.a, &self.set_h, &self.set_k);
            let points_val_p_a = get_matrix_point_val(
                &self.matrices.a,
                &self.set_h,
                &self.set_k,
                &points_row_p_a,
                &points_col_p_a,
            );

            // Matrix B:
            let points_row_p_b = get_matrix_point_row(&self.matrices.b, &self.set_h, &self.set_k);
            let points_col_p_b = get_matrix_point_col(&self.matrices.b, &self.set_h, &self.set_k);
            let points_val_p_b = get_matrix_point_val(
                &self.matrices.b,
                &self.set_h,
                &self.set_k,
                &points_row_p_b,
                &points_col_p_b,
            );

            // Matrix C
            let points_row_p_c = get_matrix_point_row(&self.matrices.c, &self.set_h, &self.set_k);
            let points_col_p_c = get_matrix_point_col(&self.matrices.c, &self.set_h, &self.set_k);
            let points_val_p_c = get_matrix_point_val(
                &self.matrices.c,
                &self.set_h,
                &self.set_k,
                &points_row_p_c,
                &points_col_p_c,
            );

            // ∑ r(alpha=10, k) * A^(k,x)
            let r_a_kx = sigma_rkx_mkx(
                &self.set_h,
                alpha,
                &points_val_p_a,
                &points_row_p_a,
                &points_col_p_a,
            );
            println!("Poly ∑ r(alpha=10, k) * A^(k,x): A_h");
            dsp_poly!(&r_a_kx);

            // ∑ r(alpha=10, k) * B^(k,x)
            let r_b_kx = sigma_rkx_mkx(
                &self.set_h,
                alpha,
                &points_val_p_b,
                &points_row_p_b,
                &points_col_p_b,
            );
            println!("Poly ∑ r(alpha=10, k) * B^(k,x): ");
            dsp_poly!(&r_b_kx);

            // ∑ r(alpha=10, k) * C^(k,x)
            let r_c_kx = sigma_rkx_mkx(
                &self.set_h,
                alpha,
                &points_val_p_c,
                &points_row_p_c,
                &points_col_p_c,
            );
            println!("Poly ∑ r(alpha=10, k) * C^(k,x): ");
            dsp_poly!(&r_c_kx);

            // ∑_m [η_M r_M(α,x)] * z^(x)
            let sum_2 = Poly::new(vec![eta_a]) * &r_a_kx
                + Poly::new(vec![eta_b]) * &r_b_kx
                + Poly::new(vec![eta_c]) * &r_c_kx;
            let sum_2 = sum_2 * &poly_z_hat_x;

            // Sum Check Protocol Formula:
            // s(x) + r(α,x) * ∑_m [η_M ​z^M​(x)] - ∑_m [η_M r_M(α,x)] * z^(x)
            let poly_scp = poly_sx.clone() + sum_1.clone() - &sum_2;

            println!("scp: ");
            dsp_poly!(&poly_scp);

            let div_res = poly_scp.div_mod(&van_poly_vhx);
            let h_1x = div_res.0;
            println!("Poly h_1x: ");
            dsp_poly!(&h_1x);

            let g_1x = div_res.1.div_mod(&Poly::new(vec![Mfp::ONE, Mfp::ZERO])).0;
            println!("Poly g_1x:");
            dsp_poly!(&g_1x);

            // TODO: Random F - H
            // let beta_1 = gen_rand_not_in_set(&vec_to_set(&self.set_h));
            let beta_1 = Mfp::from(sha2_hash(&poly_sx.eval(Mfp::from(9)).to_string()));

            // ∑ r(alpha=10, k) * A^(x,k)
            let r_a_xk = m_xk(
                &beta_1,
                &points_val_p_a,
                &points_row_p_a,
                &points_col_p_a,
                self.set_k_len.try_into().unwrap(),
            );
            println!("Poly ∑ r(alpha=10, k) * A^(x,k): ");
            dsp_poly!(&r_a_xk);

            // ∑ r(alpha=10, k) * B^(x,k)
            let r_b_xk = m_xk(
                &beta_1,
                &points_val_p_b,
                &points_row_p_b,
                &points_col_p_b,
                self.set_k_len.try_into().unwrap(),
            );
            println!("Poly ∑ r(alpha=10, k) * B^(x,k): ");
            dsp_poly!(&r_b_xk);

            // ∑ r(alpha=10, k) * C^(x,k)
            let r_c_xk = m_xk(
                &beta_1,
                &points_val_p_c,
                &points_row_p_c,
                &points_col_p_c,
                self.set_k_len.try_into().unwrap(),
            );
            println!("Poly ∑ r(alpha=10, k) * C^(x,k): ");
            dsp_poly!(&r_c_xk);

            // sigma_2
            let sigma_2 = eta_a * r_a_kx.eval(beta_1)
                + eta_b * r_b_kx.eval(beta_1)
                + eta_c * r_c_kx.eval(beta_1);
            println!("sigma_2: {}", sigma_2);

            // r(alpha=10, x) ∑_m [​η_M ​M^(x,β1​)]
            let poly_sigma_2 = Poly::new(vec![eta_a]) * r_a_xk
                + Poly::new(vec![eta_b]) * r_b_xk
                + Poly::new(vec![eta_c]) * r_c_xk;
            let poly_sigma_2 = &poly_r * poly_sigma_2;

            println!("r(alpha=10, x) * ∑_m [η_M M^(x, β1)]: ");
            dsp_poly!(&poly_sigma_2);

            let div_res = poly_sigma_2.div_mod(&van_poly_vhx);
            let h_2x = div_res.0;
            println!("Poly h_2x: ");
            dsp_poly!(&h_2x);

            let g_2x = div_res.1.div_mod(&Poly::new(vec![Mfp::ONE, Mfp::ZERO])).0;
            println!("Poly g_2x:");
            dsp_poly!(&g_2x);

            let a_row_px = sigma_yi_li(&points_row_p_a, &self.set_k);
            println!("a_row_px: ");
            dsp_poly!(&a_row_px);
            let a_col_px = sigma_yi_li(&points_col_p_a, &self.set_k);
            println!("a_col_px: ");
            dsp_poly!(&a_col_px);
            let a_val_px = sigma_yi_li(&points_val_p_a, &self.set_k);
            println!("a_val_px: ");
            dsp_poly!(&a_val_px);

            let b_row_px = sigma_yi_li(&points_row_p_b, &self.set_k);
            println!("b_row_px: ");
            dsp_poly!(&b_row_px);
            let b_col_px = sigma_yi_li(&points_col_p_b, &self.set_k);
            println!("b_col_px: ");
            dsp_poly!(&b_col_px);
            let b_val_px = sigma_yi_li(&points_val_p_b, &self.set_k);
            println!("b_val_px: ");
            dsp_poly!(&b_val_px);

            let c_row_px = sigma_yi_li(&points_row_p_c, &self.set_k);
            println!("c_row_px: ");
            dsp_poly!(&c_row_px);
            let c_col_px = sigma_yi_li(&points_col_p_c, &self.set_k);
            println!("c_col_px: ");
            dsp_poly!(&c_col_px);
            let c_val_px = sigma_yi_li(&points_val_p_c, &self.set_k);
            println!("c_val_px: ");
            dsp_poly!(&c_val_px);

            let concat_str = format!("{}{}{}{}{}", "zkIoT", "MultiSensor", "1.0", "1.0", "");
            let _ = store_commit_json(&[&a_row_px, &a_col_px, &a_val_px, &b_row_px, &b_col_px, &b_val_px, &c_row_px, &c_col_px, &c_val_px, &poly_sx], self.set_k_len as usize, self.set_h_len as usize, [self.set_h.clone(), self.set_k.clone()]);
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
}
