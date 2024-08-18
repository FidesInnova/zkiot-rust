// TODO: Check all random gens 
use ark_ff::{Field, PrimeField};
use nalgebra::{Const, DMatrix, DVector};
use parser::parse_from_lines;
use rand::thread_rng;
use rustnomial::{Degree, SizedPolynomial, Term};
use rustnomial::{Evaluable, FreeSizePolynomial, Polynomial};
use std::process::exit;
use std::{collections::HashMap, path::PathBuf, u64};
use zk_iot::*;
use anyhow::Result;
use anyhow::anyhow;

fn main() -> Result<()> {
    println!("Phase 1: Setup");
    // Phase 1: Setup
    // Initialize parameters
    let ng      = 3;            // Number of gates
    let no      = 1;            // Number of outputs
    let ni      = 1;            // Number of inputs (registers)
    let t       = ni + 1;       // Number of rows (|x| = t, where t = ni + 1)
    let g       = 2;            // Generator number
    let d       = 111213119_u64; // Large constant value
    let l: u64  = 8;            // Base exponent for sequence

    // Initialize matrices (A, B, C) with elements from finite field P
    let size = ng + ni + 1;  // Size of the matrices
    let mut a_matrix = DMatrix::<Mfp>::zeros(size, size);
    let mut b_matrix = DMatrix::<Mfp>::zeros(size, size);
    let mut c_matrix = DMatrix::<Mfp>::zeros(size, size);

    // Initialize the polynomial z with size elements, starting with 1
    let mut z_poly  = DVector::<Mfp>::zeros(size);
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
    println!("Phase 1: Commit"); 
    
    let n = 5; // Define the parameter for set H
    let m = 9; // Define the parameter for set K
    
    let generator_h = to_bint!(exp_mod(g, (P - 1) / n)); // Compute the generator for set H
    let generator_k = to_bint!(exp_mod(g, (P - 1) / m)); // Compute the generator for set K
    
    let set_h = generate_set(generator_h, n); 
    let set_k = generate_set(generator_k, m); 
    
    println!("H:\t{{ {} }}\nK:\t{{ {} }}", dsp_vec!(set_h), dsp_vec!(set_k)); // Display sets H and K
    

    // A matrix processing
    // TODO: encode_mat_m(&a_matrix, &set_h, &set_k)
    println!("A mat:");                                             
    let points  = get_points_row(&a_matrix, &set_h, &set_k);   // Get points for rows of matrix A
    let a_row   = lagrange_interpolate(&points);                    // Compute Lagrange interpolation for rows
    println!("lag row:");                                           
    dsp_poly!(a_row);                                               
    let points  = get_points_col(&a_matrix, &set_h, &set_k);   // Get points for columns of matrix A
    let a_col   = lagrange_interpolate(&points);                    // Compute Lagrange interpolation for columns
    println!("lag col:");                                           
    dsp_poly!(a_col);                                               
    let points  = get_points_val(&a_matrix, &set_h, &set_k);   // Get points for values of matrix A
    let a_val   = lagrange_interpolate(&points);                    // Compute Lagrange interpolation for values
    println!("lag val:");                                           
    dsp_poly!(a_val);                                               
    let a_matrix_encode = vec![a_row, a_col, a_val];                // Combine row, column, and value polynomials into a vector
    
    // B matrix processing
    println!("B mat:");                                             
    let points  = get_points_row(&b_matrix, &set_h, &set_k);   // Get points for rows of matrix B
    let b_row   = lagrange_interpolate(&points);                    // Compute Lagrange interpolation for rows
    println!("lag row:");                                           
    dsp_poly!(b_row);                                               
    let points  = get_points_col(&b_matrix, &set_h, &set_k);   // Get points for columns of matrix B
    let b_col   = lagrange_interpolate(&points);                    // Compute Lagrange interpolation for columns
    println!("lag col:");                                           
    dsp_poly!(b_col);                                               
    let points  = get_points_val(&b_matrix, &set_h, &set_k);   // Get points for values of matrix B
    let b_val   = lagrange_interpolate(&points);                    // Compute Lagrange interpolation for values
    println!("lag val:");                                           
    dsp_poly!(b_val);                                               
    let b_matrix_encode = vec![b_row, b_col, b_val];                // Combine row, column, and value polynomials into a vector for matrix B

    // C matrix processing
    println!("C mat");                                              
    let points  = get_points_row(&c_matrix, &set_h, &set_k);   // Get points for rows of matrix C
    let c_row   = lagrange_interpolate(&points);                    // Compute Lagrange interpolation for rows
    println!("lag row:");                                           
    dsp_poly!(c_row);                                               
    let points  = get_points_col(&c_matrix, &set_h, &set_k);   // Get points for columns of matrix C
    let c_col   = lagrange_interpolate(&points);                    // Compute Lagrange interpolation for columns
    println!("lag col:");                                           
    dsp_poly!(c_col);                                               
    let points  = get_points_val(&c_matrix, &set_h, &set_k);   // Get points for values of matrix C
    let c_val   = lagrange_interpolate(&points);                    // Compute Lagrange interpolation for values
    println!("lag val:");                                           
    dsp_poly!(c_val);                                               
    let c_matrix_encode = vec![c_row, c_col, c_val];                // Combine row, column, and value polynomials into a vector for matrix C

    // Combine encoded matrix polynomials
    let mut o_i = vec![];

    // Append encoded matrices
    o_i.extend(a_matrix_encode); // Add encoded polynomials for matrix A
    o_i.extend(b_matrix_encode); // Add encoded polynomials for matrix B
    o_i.extend(c_matrix_encode); // Add encoded polynomials for matrix C

    let commit_res = commit(o_i, d, g);                // Generate the commitment
    println!("Commit:\t( {} )", dsp_vec!(commit_res)); // Display the commitment

    // TODO: For debugging purposes.(this line will be removed)
    assert_eq!(commit_res, vec![Mfp::from(32), Mfp::from(56), Mfp::from(2), Mfp::from(135), Mfp::from(3), Mfp::from(50), Mfp::from(32), Mfp::from(32), Mfp::from(2)]);

    // Phase 3: Eval
    println!();
    println!("Phase 3: Eval");

    // This section is currently incomplete and will be finalized in the future.
    /*
    let seq_k       = generate_set_eval(generator_h, n as usize, t, set_k.len());
    let points_h    = get_points_set(&seq_k, &set_k);
    let lag_h       = lagrange_interpolate(&points_h);

    println!("Lag h:");
    dsp_poly!(lag_h);
    let a = [set_h[2], Mfp::ZERO];
    let r = set_h[1];
    let c = [
        (n as isize - t as isize),
        m as isize - (n as isize - t as isize),
    ];
    let p = [0, 3];
    points_h.iter().find(|v| **v == (Mfp::ONE, Mfp::ONE));
    let res = a.iter().zip(p.iter()).all(|(a_i, p_i)| {
        let e = exp_mod(generator_k, *p_i);
        points_h.iter().any(|v| v.0 == e && v.1 == *a_i)
    });
    println!("Geo Seq Test result:\t{:?}", res);

    let mut res = false;

    for &k in &set_k {
        if let Some((_, h_gamma_k)) = points_h
            .iter()
            .find(|v| v.0 == k * Mfp::from(generator_k))
        {
            if let Some((_, h_k)) = points_h.iter().find(|v| v.0 == k) {
                if *h_gamma_k == *h_k * r {
                    res = true;
                    break;
                }
            }
        }
        for (p_i, c_i) in p.iter().zip(c.iter()) {
            if k == exp_mod(generator_k, *p_i + *c_i as u64 - 1) {
                res = true;
                break;
            }
        }
    }
    println!("Zero Over K result:\t{:?}", res);
    let seq_kh = [1, 56, 59, 46, 42];
    */


    // Compute matrix multiplications for A, B, and C with z_poly
    let az = &a_matrix * &z_poly;
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

    // TODO: Define a random value b within the range F(0-181) and ensure 0 < b <= P - n
    let b = 2;
    // Uncomment and adjust the line below to push random points 
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
    let poly_za = lagrange_interpolate(&points_za);
    println!("^za(x):");
    dsp_poly!(poly_za);
    let poly_zb = lagrange_interpolate(&points_zb);
    println!("^zb(x):");
    dsp_poly!(poly_zb);
    let poly_zc = lagrange_interpolate(&points_zc);
    println!("^zc(x):");
    dsp_poly!(poly_zc);


    // Split set_h into two subsets based on index t
    let set_h_1 = &set_h[0..t].to_vec();    // H[>∣x∣]
    let set_h_2 = &set_h[t..].to_vec();     // H[<=∣x∣]

    // Interpolate polynomial for x^(h) over the subset H[>∣x∣]
    let z_vec       = &mat_to_vec(&z_poly);
    let points      = get_points_set(&z_vec[0..t].to_vec(), set_h_1);
    let poly_x_hat  = lagrange_interpolate(&points);

    // Interpolate polynomial w(h) over the subset H[<=∣x∣]
    let z_vec   = &mat_to_vec(&z_poly);
    let points  = get_points_set(&z_vec[t..].to_vec(), set_h_2);
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
    let poly_wh = lagrange_interpolate(&points_w);

    println!("w_hat:"); // Output the interpolated polynomial for wˉ(h)
    dsp_poly!(poly_wh);

    // h_zero
    let van_poly_vhx = vanishing_poly(&set_h);
    let poly_h_0 = (&poly_za * &poly_zb - &poly_zc).div_mod(&van_poly_vhx);
    
    println!("h0(x):");
    dsp_poly!(poly_h_0.0);

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

    // Hardcoded values for demonstration
    let alpha = Mfp::from(10);
    let eta_a = Mfp::from(2);
    let eta_b = Mfp::from(30);
    let eta_c = Mfp::from(100);

    // Compute polynomial for ∑ ηz(x)
    let sigma_eta_z_x = Polynomial::new(vec![eta_a]) * &poly_za +
                        Polynomial::new(vec![eta_b]) * &poly_zb + 
                        Polynomial::new(vec![eta_c]) * &poly_zc;

    // Compute polynomial for r(α,x) ∑ ηM(z^M(x))

    let poly_r = func_u(Some(alpha), None, set_h.len());
    println!("r:");
    dsp_poly!(poly_r);

    println!("r(α,x) ∑( η_M z^_M(x) ):");
    dsp_poly!((&poly_r * &sigma_eta_z_x));

    // r(α,x) * ∑_m [η_M ​z^M​(x)]
    let sum_1 = &poly_r * sigma_eta_z_x;



    // Compute polynomial for Z^(x)
    let poly_z_hat_x = poly_wh * van_poly_vh1 + poly_x_hat; 
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
    let poly_scp = poly_sx + sum_1 - &sum_2;

    println!("scp: ");
    dsp_poly!(poly_scp);



    let div_res = poly_scp.div_mod(&van_poly_vhx);
    let h_1x = div_res.0;
    println!("Poly h_1x: ");
    dsp_poly!(h_1x);

    let g_1x = div_res.1.div_mod(&Poly::new(vec![Mfp::ONE, Mfp::ZERO])).0;
    println!("Poly g_1x: {}", sigma_1/Mfp::from(set_h.len() as u64));
    dsp_poly!(g_1x);

    // TODO: Random F - H 
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
    let sigma_2 =   Poly::new(vec![eta_a]) * r_a_kx.eval(beta_1) + 
                    Poly::new(vec![eta_b]) * r_b_kx.eval(beta_1) +
                    Poly::new(vec![eta_c]) * r_c_kx.eval(beta_1);
    let sigma_2 = sigma_2.eval(beta_1);
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
    println!("Poly g_2x: {}", sigma_1/Mfp::from(set_h.len() as u64));
    dsp_poly!(g_2x);

    // TODO: Random F - H 
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
        let sig_a = sigma_m(&van_poly_vhx, &eta_a, &beta_1, &beta_2, k, &a_row_px, &a_col_px, &a_val_px);
        let sig_b = sigma_m(&van_poly_vhx, &eta_b, &beta_1, &beta_2, k, &b_row_px, &b_col_px, &b_val_px);
        let sig_c = sigma_m(&van_poly_vhx, &eta_c, &beta_1, &beta_2, k, &c_row_px, &c_col_px, &c_val_px);

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

    println!("a(x): ");
    dsp_poly!(poly_a_x);
    
    // b(x)
    let poly_b_x = &poly_pi_a * &poly_pi_b * &poly_pi_c; 

    println!("b(x): ");
    dsp_poly!(poly_b_x);

    // let h_3x = 1;
    // let g_3x = 1;
    Ok(())
}


fn sigma_m(van_poly_vhx: &Poly, eta: &Mfp, beta_1: &Mfp, beta_2: &Mfp, k: &Mfp, row: &Poly, col: &Poly, val: &Poly) -> Mfp {
    let nu = van_poly_vhx.eval(*beta_1) * van_poly_vhx.eval(*beta_2) * val.eval(*k);
    let de = (beta_2 - &row.eval(*k)) * (beta_1 - &col.eval(*k));
    let div = nu / de;
    let sig = eta * &div;
    sig
}

pub fn sigma_yi_li(points: &HashMap<Mfp, Mfp>, set_k: &Vec<Mfp>) -> Poly {
    let mut points_li: Vec<Point> = vec![];
    for k in set_k {
        let val = points.get(k).unwrap_or(&Mfp::ZERO);
        points_li.push((*k, *val));
    }
    lagrange_interpolate(&points_li)
}

pub fn sigma_rkx_mkx(set_h: &Vec<Mfp>, alpha: Mfp, points_val: &HashMap<Mfp, Mfp>, points_row: &HashMap<Mfp, Mfp>, points_col: &HashMap<Mfp, Mfp>) -> Poly {
    let mut res = Poly::from(vec![Mfp::ZERO]);
    for h in set_h {
        let mut p_r_alphak = func_u(Some(alpha), Some(*h), set_h.len());
        let mut p_m_kx = m_kx(h, &points_val, &points_row, &points_col, set_h.len());

        p_r_alphak.trim();
        p_m_kx.trim();

        // dsp_poly!(p_m_kx);

        res += p_r_alphak * p_m_kx;
    }
    res
}


pub fn m_kx(num: &Mfp, points_val: &HashMap<Mfp, Mfp>, points_row: &HashMap<Mfp, Mfp>, points_col: &HashMap<Mfp, Mfp>, set_h_len: usize) -> Poly {
    let mut poly_res = Poly::from(vec![Mfp::ZERO]);
    
    for (k, val) in points_val {
        let poly_val = Poly::from(vec![*val]);
        let poly_x = func_u(None, Some(points_row[k]), set_h_len);
        let poly_y = func_u(None, Some(points_col[k]), set_h_len);
        let res_poly_x = poly_x.eval(*num);
        poly_res += poly_val * res_poly_x * poly_y;
    }

    poly_res
}



pub fn sigma_rxk_mxk(set_h: &Vec<Mfp>, alpha: Mfp, points_val: &HashMap<Mfp, Mfp>, points_row: &HashMap<Mfp, Mfp>, points_col: &HashMap<Mfp, Mfp>) -> Poly {
    let mut res = Poly::from(vec![Mfp::ZERO]);
    
    for h in set_h {
        let mut p_r_alphak = func_u(Some(alpha), Some(*h), set_h.len());
        let mut p_m_xk = m_xk(h, &points_val, &points_row, &points_col, set_h.len());

        p_r_alphak.trim();
        p_m_xk.trim();

        // dsp_poly!(p_m_kx);

        res += p_r_alphak * p_m_xk;
    }

    res
} 

pub fn m_xk(num: &Mfp, points_val: &HashMap<Mfp, Mfp>, points_row: &HashMap<Mfp, Mfp>, points_col: &HashMap<Mfp, Mfp>, set_h_len: usize) -> Poly {
    let mut poly_res = Poly::from(vec![Mfp::ZERO]);
    
    for (k, val) in points_val {
        let poly_val = Poly::from(vec![*val]);
        let poly_x = func_u(None, Some(points_row[k]), set_h_len);
        let poly_y = func_u(None, Some(points_col[k]), set_h_len);
        let res_poly_y = poly_y.eval(*num);
        poly_res += poly_val * res_poly_y * poly_x;
    }

    poly_res
}


pub fn get_matrix_point_row(mat: &DMatrix<Mfp>, set_h: &Vec<Mfp>, set_k: &Vec<Mfp>) -> HashMap<Mfp, Mfp> {
    let mut res = HashMap::new();
    let mut c = 0;
    let mat_len = mat.nrows();

    for i in 0..mat_len {
        for j in 0..mat_len {
            if mat[(i, j)] != Mfp::ZERO {
                res.insert(set_k[c], set_h[i]);
                c += 1;
            }
        }
    }
    res 
}

pub fn get_matrix_point_col(mat: &DMatrix<Mfp>, set_h: &Vec<Mfp>, set_k: &Vec<Mfp>) -> HashMap<Mfp, Mfp> {
    let mut res = HashMap::new();
    let mut c = 0;
    let mat_len = mat.nrows();

    for i in 0..mat_len {
        for j in 0..mat_len {
            if mat[(i, j)] != Mfp::ZERO {
                res.insert(set_k[c], set_h[j]);
                c += 1;
            }
        }
    }

    res 
}


pub fn get_matrix_point_val(mat: &DMatrix<Mfp>, set_h: &Vec<Mfp>, set_k: &Vec<Mfp>, row_k: &HashMap<Mfp, Mfp>, col_k: &HashMap<Mfp, Mfp>) -> HashMap<Mfp, Mfp> {
    let mut res = HashMap::new();
    let mut c = 0;
    let mat_len = mat.nrows();

    let len = set_h.len();
    let mut poly_u = Poly::from(vec![Mfp::ZERO]);
    poly_u.add_term(Mfp::from(len as u64), len - 1);

    for i in 0..mat_len {
        for j in 0..mat_len {
            if mat[(i, j)] != Mfp::ZERO {
                let val = mat[(i, j)];
                let k   = set_k[c];
                let p2  =  val / (poly_u.eval(row_k[&k]) * poly_u.eval(col_k[&k]));
                res.insert(set_k[c], p2);
                c += 1;
            }
        }
        println!();
    }
    
    res 
}