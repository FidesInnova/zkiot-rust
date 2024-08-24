use rustnomial::{Evaluable, Polynomial};
use std::path::PathBuf;
use nalgebra::DMatrix;
use anyhow::Result;
use ark_ff::Field;

use parser::parse_from_lines;
use zk_iot::*;
use utils::*;
use math::*;
use file::*;

fn main() -> Result<()> {
    // clear json file
    clean_files()?;
    println!("Phase 1: Setup");
    // Phase 1: Setup
    // Initialize parameters
    let ng      = 3;            // Number of gates
    let _no     = 1;            // Number of outputs
    let ni      = 1;            // Number of inputs (registers)
    let t       = ni + 1;       // Number of rows (|x| = t, where t = ni + 1)
    let g       = 2;            // Generator number
    let d       = 111213119_u64;// Large constant value
    let l: u64  = 8;            // Base exponent for sequence

    // Initialize matrices (A, B, C) with elements from finite field P
    let size = ng + ni + 1;  // Size of the matrices
    let mut a_matrix = DMatrix::<Mfp>::zeros(size, size);
    let mut b_matrix = DMatrix::<Mfp>::zeros(size, size);
    let mut c_matrix = DMatrix::<Mfp>::zeros(size, size);

    // Initialize the polynomial z with size elements, starting with 1
    let mut z_poly  = DMatrix::<Mfp>::zeros(size, 1);
    z_poly[0]       = Mfp::ONE;

    // TODO: Update this part after the parser has been updated!
    // 'Load number' 
    // Set the second element of z_poly to 4 'R1(1) = 4'
    let r1      = Mfp::from(4);
    z_poly[1]   = r1;

    // Parse gate definitions from file
    let gates   = parse_from_lines(
        &PathBuf::from("line_num.txt"), 
        &PathBuf::from("sample.txt")
    )?;

    // Initialize matrices A, B, C and polynomial z with parsed gates
    init(
        gates,
        ni,
        &mut a_matrix,
        &mut b_matrix,
        &mut c_matrix,
        &mut z_poly,
    );

    // Set specific rows in matrices A, B, C to zero
    rows_to_zero(&mut a_matrix, t);
    rows_to_zero(&mut b_matrix, t);
    rows_to_zero(&mut c_matrix, t);

    // Calculate Cz = (A * z) . (B * z)
    let cz = (&a_matrix * &z_poly).component_mul(&(&b_matrix * &z_poly));

    // Display matrices A, B, C, and Cz for verification
    println!("A:");
    dsp_mat!(&a_matrix);
    println!("B:");
    dsp_mat!(&b_matrix);
    println!("C:");
    dsp_mat!(&c_matrix);
    println!("Cz:");
    dsp_mat!(cz);

    // Generate the proof path by iteratively applying exponentiation
    let mut proof_path = vec![];
    let mut s = Mfp::from(g);
    let d = d % (P - 1);
    for _ in 0..=l {
        proof_path.push(s);
        s = exp_mod(to_bint!(s), d);
    }

    println!(); 
    println!("Proof Path:\t( {} )", dsp_vec!(proof_path)); 

    // Phase 2: Commit 
    println!();
    println!("Phase 2: Commit"); 
    
    let n = 5; // Define the parameter for set H
    let m = 9; // Define the parameter for set K
    
    let generator_h = to_bint!(exp_mod(g, (P - 1) / n)); // Compute the generator for set H
    let generator_k = to_bint!(exp_mod(g, (P - 1) / m)); // Compute the generator for set K
    
    let set_h = generate_set(generator_h, n); 
    let set_k = generate_set(generator_k, m); 
    
    println!("H:\t{{ {} }}\nK:\t{{ {} }}", dsp_vec!(set_h), dsp_vec!(set_k)); // Display sets H and K

    // A matrix processing
    // println!("A mat:");                                           
    let a_matrix_encode = encode_matrix_m(&a_matrix, &set_h, &set_k);
    
    // B matrix processing
    // println!("B mat:");                              
    let b_matrix_encode = encode_matrix_m(&b_matrix, &set_h, &set_k);

    // C matrix processing
    // println!("C mat");                                     
    let c_matrix_encode = encode_matrix_m(&c_matrix, &set_h, &set_k);

    // Combine encoded matrix polynomials
    let mut o_i = vec![];

    // Append encoded matrices
    o_i.extend(a_matrix_encode); // Add encoded polynomials for matrix A
    o_i.extend(b_matrix_encode); // Add encoded polynomials for matrix B
    o_i.extend(c_matrix_encode); // Add encoded polynomials for matrix C

    let commit_res = commit(&o_i, d, g);               // Generate the commitment
    println!("Commit:\t( {} )", dsp_vec!(commit_res)); // Display the commitment

    // Phase 3: Eval
    println!();
    println!("Phase 3: Eval");


    // Compute matrix multiplications for A, B, and C with z_poly
    let az: DMatrix<Mfp> = &a_matrix * &z_poly;
    let bz = &b_matrix * &z_poly;
    let cz = &c_matrix * &z_poly;

    // Display the results of matrix multiplications
    println!("Matrix Az:");
    dsp_mat!(az);
    println!("Matrix Bz");
    dsp_mat!(bz);
    println!("Matrix Cz");
    dsp_mat!(cz);

    // Convert matrices to vectors and retrieve corresponding points
    let mut points_za = get_points_set(&mat_to_vec(&az), &set_h);
    let mut points_zb = get_points_set(&mat_to_vec(&bz), &set_h);
    let mut points_zc = get_points_set(&mat_to_vec(&cz), &set_h);


    // Uncomment and adjust the line below to push random points 
    // TODO: Define a random value b within the range F(0..P-n) and ensure 0 < b <= P - n
    let b = 2;
    // push_random_points(&mut points_za, b, &vec_to_hashset(&set_h));
    // push_random_points(&mut points_zb, b, &vec_to_hashset(&set_h));
    // push_random_points(&mut points_zc, b, &vec_to_hashset(&set_h));

    // Add random interpolations for za
    points_za.push((Mfp::from(150), Mfp::from(5)));
    points_za.push((Mfp::from(80), Mfp::from(47)));

    // Add random interpolations for zb
    points_zb.push((Mfp::from(150), Mfp::from(15)));
    points_zb.push((Mfp::from(80), Mfp::from(170)));

    // Add random interpolations for zc
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
    let set_h_1 = &set_h[0..t].to_vec();    // H[>∣x∣]
    let set_h_2 = &set_h[t..].to_vec();     // H[<=∣x∣]

    // Interpolate polynomial for x^(h) over the subset H[>∣x∣]
    let z_vec       = &mat_to_vec(&z_poly);
    let points      = get_points_set(&z_vec[0..t], set_h_1);
    let poly_x_hat  = lagrange_interpolate(&points);

    // Interpolate polynomial w(h) over the subset H[<=∣x∣]
    let z_vec   = &mat_to_vec(&z_poly);
    let points  = get_points_set(&z_vec[t..], set_h_2);
    let wh      = lagrange_interpolate(&points);

    // Compute the vanishing polynomial for the subset H[<=∣x∣]
    let van_poly_vh1 = vanishing_poly(set_h_1);

    let mut points_w = vec![];
    for i in set_h_2 {
        // Compute the adjusted polynomial wˉ(h) for each element in the subset
        let w_bar_h = (wh.eval(*i) - poly_x_hat.eval(*i)) * van_poly_vh1.eval(*i).inverse().unwrap();
        points_w.push((*i, w_bar_h));
    }

    // Uncomment these lines to insert random points for wˉ(h) from the set
    // push_random_points(&mut points_w, b, &vec_to_hashset(&set_h));

    // Insert example points into points_w for wˉ(h)
    points_w.push((Mfp::from(150), Mfp::from(42)));
    points_w.push((Mfp::from(80), Mfp::from(180)));

    // Interpolate polynomial for wˉ(h) based on the points_w
    let poly_w_hat = lagrange_interpolate(&points_w);

    println!("w_hat:"); // Output the interpolated polynomial for wˉ(h)
    dsp_poly!(poly_w_hat);

    // h_zero
    let van_poly_vhx = vanishing_poly(&set_h);
    let poly_h_0 = (&poly_z_hat_a * &poly_z_hat_b - &poly_z_hat_c).div_mod(&van_poly_vhx).0;
    
    println!("h0(x):");
    dsp_poly!(poly_h_0);

    // Generate a random polynomial (currently hardcoded for demonstration)
    let poly_sx = [5, 0, 101, 17, 0, 1, 20, 0, 0, 3, 115];
    let poly_sx = poly_sx.iter().map(|v| Mfp::from(*v)).collect::<Vec<Mfp>>();
    let poly_sx = Polynomial::from(poly_sx);

    // Compute sigma by evaluating the polynomial at points in set_h
    let sigma_1 = set_h.iter().fold(Mfp::ZERO, |acc, &v| acc + poly_sx.eval(v));
    println!("sigma_1 :\t{}", sigma_1);

    // TODO: Uncomment this lines
    // Generate random values for alpha, eta_a, eta_b, eta_c
    // let alpha = MFp::from(thread_rng().gen_range(0..P));
    // let eta_a = MFp::from(thread_rng().gen_range(0..P));
    // let eta_b = MFp::from(thread_rng().gen_range(0..P));
    // let eta_c = MFp::from(thread_rng().gen_range(0..P));

    // let alpha = sip_hash(&(poly_sx.eval(Mfp::from(0)) + poly_sx.eval(Mfp::from(1)) + Mfp::from(1)));
    // let eta_a = sip_hash(&(poly_sx.eval(Mfp::from(2)) + poly_sx.eval(Mfp::from(3)) + Mfp::from(2)));
    // let eta_b = sip_hash(&(poly_sx.eval(Mfp::from(4)) + poly_sx.eval(Mfp::from(5)) + Mfp::from(3)));
    // let eta_c = sip_hash(&(poly_sx.eval(Mfp::from(6)) + poly_sx.eval(Mfp::from(7)) + Mfp::from(4)));

    // Hardcoded values for test
    let alpha = Mfp::from(10);
    let eta_a = Mfp::from(2);
    let eta_b = Mfp::from(30);
    let eta_c = Mfp::from(100);

    // Compute polynomial for ∑ ηz(x)
    let sigma_eta_z_x = Polynomial::new(vec![eta_a]) * &poly_z_hat_a +
                        Polynomial::new(vec![eta_b]) * &poly_z_hat_b + 
                        Polynomial::new(vec![eta_c]) * &poly_z_hat_c;

    // Compute polynomial for r(α,x) ∑ ηM(z^M(x))
    let poly_r = func_u(Some(alpha), None, set_h.len());
    println!("r:");
    dsp_poly!(poly_r);

    println!("r(alpha , x) ∑_m [η_M z^_M(x)]:");
    dsp_poly!((&poly_r * &sigma_eta_z_x));

    // r(α,x) * ∑_m [η_M ​z^M​(x)]
    let sum_1 = &poly_r * sigma_eta_z_x;

    // Compute polynomial for Z^(x)
    let poly_z_hat_x = &poly_w_hat * &van_poly_vh1 + poly_x_hat; 
    println!("z_hat: ");
    dsp_poly!(poly_z_hat_x);

    // Matrix A: 
    let mut points_row_p_a = get_matrix_point_row(&a_matrix, &set_h, &set_k);
    let points_add = vec![
        (Mfp::from(48), Mfp::from(1)),
        (Mfp::from(73), Mfp::from(135)),
        (Mfp::from(62), Mfp::from(125)),
        (Mfp::from(132), Mfp::from(59)),
        (Mfp::from(65), Mfp::from(42)),
        (Mfp::from(80), Mfp::from(1)),
    ];
    for (k, v) in points_add {
        points_row_p_a.insert(k, v);
    }
    // println!("{:?}", points_row_p_a);

    let mut points_col_p_a = get_matrix_point_col(&a_matrix, &set_h, &set_k);
    let points_add = vec![
        (Mfp::from(48), Mfp::from(42)),
        (Mfp::from(73), Mfp::from(1)),
        (Mfp::from(62), Mfp::from(135)),
        (Mfp::from(132), Mfp::from(125)),
        (Mfp::from(65), Mfp::from(59)),
        (Mfp::from(80), Mfp::from(42)),
    ];
    for (k, v) in points_add {
        points_col_p_a.insert(k, v);
    }
    // println!("{:?}", points_col_p_a);

    let points_val_p_a = get_matrix_point_val(&a_matrix, &set_h, &set_k, &points_row_p_a, &points_col_p_a);
    // println!("{:?}", points_val_p_a);

    // Matrix B: 
    let mut points_row_p_b = get_matrix_point_row(&b_matrix, &set_h, &set_k);
    let points_add = vec![
        (Mfp::from(73), Mfp::from(59)),
        (Mfp::from(62), Mfp::from(1)),
        (Mfp::from(132), Mfp::from(42)),
        (Mfp::from(65), Mfp::from(135)),
        (Mfp::from(80), Mfp::from(59)),
    ];
    for (k, v) in points_add {
        points_row_p_b.insert(k, v);
    }
    // println!("{:?}", points_row_p_b);

    let mut points_col_p_b = get_matrix_point_col(&b_matrix, &set_h, &set_k);
    let points_add = vec![
        (Mfp::from(73), Mfp::from(59)),
        (Mfp::from(62), Mfp::from(42)),
        (Mfp::from(132), Mfp::from(125)),
        (Mfp::from(65), Mfp::from(1)),
        (Mfp::from(80), Mfp::from(135)),
    ];
    for (k, v) in points_add {
        points_col_p_b.insert(k, v);
    }
    // println!("{:?}", points_col_p_b);

    let points_val_p_b = get_matrix_point_val(&b_matrix, &set_h, &set_k, &points_row_p_b, &points_col_p_b);
    // println!("{:?}", points_val_p_a);


    // Matrix C: 
    let mut points_row_p_c = get_matrix_point_row(&c_matrix, &set_h, &set_k);
    let points_add = vec![
        (Mfp::from(48), Mfp::from(1)),
        (Mfp::from(73), Mfp::from(59)),
        (Mfp::from(62), Mfp::from(125)),
        (Mfp::from(132), Mfp::from(1)),
        (Mfp::from(65), Mfp::from(135)),
        (Mfp::from(80), Mfp::from(42)),
    ];
    for (k, v) in points_add {
        points_row_p_c.insert(k, v);
    }
    // println!("{:?}", points_row_p_c);

    let mut points_col_p_c = get_matrix_point_col(&c_matrix, &set_h, &set_k);
    let points_add = vec![
        (Mfp::from(48), Mfp::from(125)),
        (Mfp::from(73), Mfp::from(59)),
        (Mfp::from(62), Mfp::from(1)),
        (Mfp::from(132), Mfp::from(1)),
        (Mfp::from(65), Mfp::from(42)),
        (Mfp::from(80), Mfp::from(59)),
    ];
    for (k, v) in points_add {
        points_col_p_c.insert(k, v);
    }
    // println!("{:?}", points_col_p_c);

    let points_val_p_c = get_matrix_point_val(&c_matrix, &set_h, &set_k, &points_row_p_c, &points_col_p_c);
    // println!("{:?}", points_val_p_a);


    // ∑ r(alpha=10, k) * A^(k,x)
    let r_a_kx = sigma_rkx_mkx(&set_h, alpha, &points_val_p_a, &points_row_p_a, &points_col_p_a);
    println!("Poly ∑ r(alpha=10, k) * A^(k,x): ");
    dsp_poly!(r_a_kx);


    // ∑ r(alpha=10, k) * B^(k,x)
    let r_b_kx = sigma_rkx_mkx(&set_h, alpha, &points_val_p_b, &points_row_p_b, &points_col_p_b);
    println!("Poly ∑ r(alpha=10, k) * B^(k,x): ");
    dsp_poly!(r_b_kx);

    // ∑ r(alpha=10, k) * C^(k,x)
    let r_c_kx = sigma_rkx_mkx(&set_h, alpha, &points_val_p_c, &points_row_p_c, &points_col_p_c);
    println!("Poly ∑ r(alpha=10, k) * C^(k,x): ");
    dsp_poly!(r_c_kx);

    // ∑_m [η_M r_M(α,x)] * z^(x)
    let sum_2 =   Poly::new(vec![eta_a]) * &r_a_kx + 
                    Poly::new(vec![eta_b]) * &r_b_kx +
                    Poly::new(vec![eta_c]) * &r_c_kx;
    let sum_2 = sum_2 * poly_z_hat_x;

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
    // let beta_1 = gen_rand_not_in_set(&vec_to_set(&set_h));
    let beta_1 = Mfp::from(22);

    // ∑ r(alpha=10, k) * A^(x,k)
    let r_a_xk = m_xk(&beta_1, &points_val_p_a, &points_row_p_a, &points_col_p_a, set_h.len());
    // println!("Poly ∑ r(alpha=10, k) * A^(x,k): ");
    // dsp_poly!(r_a_xk);

    // ∑ r(alpha=10, k) * B^(x,k)
    let r_b_xk = m_xk(&beta_1, &points_val_p_b, &points_row_p_b, &points_col_p_b, set_h.len());
    // println!("Poly ∑ r(alpha=10, k) * B^(x,k): ");
    // dsp_poly!(r_b_xk);

    // ∑ r(alpha=10, k) * C^(x,k)
    let r_c_xk = m_xk(&beta_1, &points_val_p_c, &points_row_p_c, &points_col_p_c, set_h.len());
    // println!("Poly ∑ r(alpha=10, k) * C^(x,k): ");
    // dsp_poly!(r_c_xk);
    

    // sigma_2
    let sigma_2 = eta_a * r_a_kx.eval(beta_1) + 
                    eta_b * r_b_kx.eval(beta_1) +
                    eta_c * r_c_kx.eval(beta_1);
    println!("sigma_2: {}", sigma_2);


    // r(alpha=10, x) ∑_m [​η_M ​M^(x,β1​)] 
    let poly_sigma_2 =  Poly::new(vec![eta_a]) * r_a_xk + 
                        Poly::new(vec![eta_b]) * r_b_xk +
                        Poly::new(vec![eta_c]) * r_c_xk;
    let poly_sigma_2 = &poly_r * poly_sigma_2;

    println!("r(alpha=10, x) * ∑_m [η_M M^(x, β1)]: ");
    dsp_poly!(poly_sigma_2);

    let div_res = poly_sigma_2.div_mod(&van_poly_vhx);
    let h_2x = div_res.0;
    println!("Poly h_2x: ");
    dsp_poly!(h_2x);

    let g_2x = div_res.1.div_mod(&Poly::new(vec![Mfp::ONE, Mfp::ZERO])).0;
    println!("Poly g_2x:");
    dsp_poly!(g_2x);

    // TODO: Random F - H 
    // let beta_2 = gen_rand_not_in_set(&vec_to_hashset(&set_h));
    let beta_2 = Mfp::from(80);


    let a_row_px = sigma_yi_li(&points_row_p_a, &set_k);
    println!("a_row_px: ");
    dsp_poly!(a_row_px);
    let a_col_px = sigma_yi_li(&points_col_p_a, &set_k);
    println!("a_col_px: ");
    dsp_poly!(a_col_px);
    let a_val_px = sigma_yi_li(&points_val_p_a, &set_k);
    println!("a_val_px: ");
    dsp_poly!(a_val_px);

    let b_row_px = sigma_yi_li(&points_row_p_b, &set_k);
    println!("b_row_px: ");
    dsp_poly!(b_row_px);
    let b_col_px = sigma_yi_li(&points_col_p_b, &set_k);
    println!("b_col_px: ");
    dsp_poly!(b_col_px);
    let b_val_px = sigma_yi_li(&points_val_p_b, &set_k);
    println!("b_val_px: ");
    dsp_poly!(b_val_px);

    let c_row_px = sigma_yi_li(&points_row_p_c, &set_k);
    println!("c_row_px: ");
    dsp_poly!(c_row_px);
    let c_col_px = sigma_yi_li(&points_col_p_c, &set_k);
    println!("c_col_px: ");
    dsp_poly!(c_col_px);
    let c_val_px = sigma_yi_li(&points_val_p_c, &set_k);
    println!("c_val_px: ");
    dsp_poly!(c_val_px);

    // sigma_3 
    let mut sigma_3 = Mfp::ZERO; 
    for k in set_k.iter() {
        let sig_a = sigma_m(&van_poly_vhx, &eta_a, &beta_1, &beta_2, k, &[&a_row_px, &a_col_px, &a_val_px]);
        let sig_b = sigma_m(&van_poly_vhx, &eta_b, &beta_1, &beta_2, k, &[&b_row_px, &b_col_px, &b_val_px]);
        let sig_c = sigma_m(&van_poly_vhx, &eta_c, &beta_1, &beta_2, k, &[&c_row_px, &c_col_px, &c_val_px]);

        sigma_3 += sig_a + sig_b + sig_c
    }
    println!("sigma_3: {}", sigma_3);
    
    // a(x) 
    let poly_pi_a = (Poly::from(vec![beta_2]) -  &a_row_px) * (Poly::from(vec![beta_1]) -  &a_col_px);
    let poly_pi_b = (Poly::from(vec![beta_2]) -  &b_row_px) * (Poly::from(vec![beta_1]) -  &b_col_px);
    let poly_pi_c = (Poly::from(vec![beta_2]) -  &c_row_px) * (Poly::from(vec![beta_1]) -  &c_col_px);

    let poly_sig_a = Poly::from(vec![eta_a * van_poly_vhx.eval(beta_2) * van_poly_vhx.eval(beta_1)]) * &a_val_px;
    let poly_sig_b = Poly::from(vec![eta_b * van_poly_vhx.eval(beta_2) * van_poly_vhx.eval(beta_1)]) * &b_val_px;
    let poly_sig_c = Poly::from(vec![eta_c * van_poly_vhx.eval(beta_2) * van_poly_vhx.eval(beta_1)]) * &c_val_px;

    let poly_a_x = poly_sig_a * (&poly_pi_b * &poly_pi_c) + 
                   poly_sig_b * (&poly_pi_a * &poly_pi_c) +
                   poly_sig_c * (&poly_pi_a * &poly_pi_b);

    println!("a(x):");
    dsp_poly!(poly_a_x);
    
    // b(x)
    let poly_b_x = &poly_pi_a * &poly_pi_b * &poly_pi_c; 

    println!("b(x): ");
    dsp_poly!(poly_b_x);

    store_commit_json(&[&a_row_px, &a_col_px, &a_val_px, &b_row_px, &b_col_px, &b_val_px, &c_row_px, &c_col_px, &c_val_px], t, size, &proof_path)?;
    store_proof_json(
        &[
            &poly_w_hat,
            &poly_z_hat_a,
            &poly_z_hat_b,
            &poly_z_hat_c,
            &poly_h_0,
            &poly_sx,
            &g_1x,
            &h_1x,
            &g_2x,
            &h_2x,
        ],
        &[&sigma_1, &sigma_2, &sigma_3],
        b, 
        set_h.len(),
        set_k.len()
    )?;
    Ok(())
}
