use commitment::Commitment;
// use rand::{thread_rng, Rng};
use rustnomial::{Evaluable, Polynomial};
use setup::Setup;
use std::{collections::HashMap, path::PathBuf, process::exit};
use nalgebra::DMatrix;
use anyhow::Result;
use ark_ff::Field;

use parser::parse_from_lines;
use zk_iot::*;
use utils::*;
use math::*;
use json_file::*;


fn main() -> Result<()> {
    let timer = std::time::Instant::now();

    let setup = Setup::new();
    let proof_path = setup.proof_path();

    println!(); 
    println!("Proof Path:\t( {} )", dsp_vec!(proof_path)); 

    // Phase 2: Commit 
    println!();
    println!("Phase 2: Commit"); 
    let mut commitment = Commitment::new(&setup)?;
    println!("H:\t{{ {} }}\nK:\t{{ {} }}", dsp_vec!(commitment.set_h), dsp_vec!(commitment.set_k)); // Display sets H and K

    
    let line_file = open_file(&PathBuf::from("line_num.txt"))?;
    // Parse gate definitions from file
    let gates = parse_from_lines(
        line_file,
        &PathBuf::from("sample.txt")
    )?;

    commitment.build_matrices(gates, setup.number_input);    

    // Calculate Cz = (A * z) . (B * z)
    let cz_matrix = (&commitment.matrices.a * &commitment.matrices.z).component_mul(&(&commitment.matrices.b * &commitment.matrices.z));

    // Display matrices A, B, C, and Cz for verification
    println!("A:");
    dsp_mat!(&commitment.matrices.a);
    println!("B:");
    dsp_mat!(&commitment.matrices.b);
    println!("C:");
    dsp_mat!(&commitment.matrices.c);
    println!("Cz:");
    dsp_mat!(cz_matrix);

    let commit_res = commitment.commit(setup.long_const_val, setup.generator); // Generate the commitment
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
    // points_za.push((Mfp::from(150), Mfp::from(5)));
    // points_za.push((Mfp::from(80), Mfp::from(47)));

    // rand Moka
    points_za.push((Mfp::from(1343080906), Mfp::from(85440204)));
    points_za.push((Mfp::from(1276955322), Mfp::from(1623925182)));

    // // Add random interpolations for zb
    // points_zb.push((Mfp::from(150), Mfp::from(15)));
    // points_zb.push((Mfp::from(80), Mfp::from(170)));

    // rand moka
    points_zb.push((Mfp::from(1343080906), Mfp::from(557672531)));
    points_zb.push((Mfp::from(1276955322), Mfp::from(879067083)));

    // // Add random interpolations for zc
    // points_zc.push((Mfp::from(150), Mfp::from(1)));
    // points_zc.push((Mfp::from(80), Mfp::from(100)));

    // rand Moka
    points_zc.push((Mfp::from(1343080906), Mfp::from(944537116)));
    points_zc.push((Mfp::from(1276955322), Mfp::from(281670719)));

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

    println!("van_poly_vh1: ");
    dsp_poly!(van_poly_vh1);

    let mut points_w = vec![];
    for i in set_h_2 {
        // Compute the adjusted polynomial wˉ(h) for each element in the subset
        let w_bar_h = (wh.eval(*i) - poly_x_hat.eval(*i)) * van_poly_vh1.eval(*i).inverse().unwrap();
        points_w.push((*i, w_bar_h));
    }

    // Uncomment these lines to insert random points for wˉ(h) from the set
    // push_random_points(&mut points_w, b, &vec_to_hashset(&set_h));

    // Insert example points into points_w for wˉ(h)
    // points_w.push((Mfp::from(150), Mfp::from(42)));
    // points_w.push((Mfp::from(80), Mfp::from(180)));


    // moka
    points_w.push((Mfp::from(1343080906), Mfp::from(944537116)));
    points_w.push((Mfp::from(1276955322), Mfp::from(281670719)));

    // Interpolate polynomial for wˉ(h) based on the points_w
    let poly_w_hat = lagrange_interpolate(&points_w);

    println!("w_hat:"); // Output the interpolated polynomial for wˉ(h)
    dsp_poly!(poly_w_hat);

    // h_zero
    let van_poly_vhx = vanishing_poly(&set_h);
    println!("van_poly_vhx: ");
    dsp_poly!(van_poly_vhx);

    let poly_ab_c = &poly_z_hat_a * &poly_z_hat_b - &poly_z_hat_c;
    let poly_h_0 = (&poly_ab_c).div_mod(&van_poly_vhx).0;
    
    println!("h0(x):");
    dsp_poly!(poly_h_0);

    // Generate a random polynomial (currently hardcoded for demonstration)
    let poly_sx = [5, 0, 101, 17, 0, 1, 20, 0, 0, 3, 115];
    let poly_sx = poly_sx.iter().map(|v| Mfp::from(*v)).collect::<Vec<Mfp>>();
    let poly_sx = Polynomial::from(poly_sx);

    println!("poly_sx");
    dsp_poly!(poly_sx);

    // Compute sigma by evaluating the polynomial at points in set_h
    let sigma_1 = set_h.iter().fold(Mfp::ZERO, |acc, &v| acc + poly_sx.eval(v));
    println!("sigma_1 :\t{}", sigma_1);

    // TODO: Uncomment this lines
    // Generate random values for alpha, eta_a, eta_b, eta_c
    // let alpha = MFp::from(thread_rng().gen_range(0..P));
    // let eta_a = MFp::from(thread_rng().gen_range(0..P));
    // let eta_b = MFp::from(thread_rng().gen_range(0..P));
    // let eta_c = MFp::from(thread_rng().gen_range(0..P));

    // let alpha = Mfp::from(sha2_hash(&to_bint!(poly_sx.eval(Mfp::from(0)) + poly_sx.eval(Mfp::from(1)) + Mfp::from(1)).to_string()));
    // let eta_a = sha2_hash(&(poly_sx.eval(Mfp::from(2)) + poly_sx.eval(Mfp::from(3)) + Mfp::from(2)));
    // let eta_b = sha2_hash(&(poly_sx.eval(Mfp::from(4)) + poly_sx.eval(Mfp::from(5)) + Mfp::from(3)));
    // let eta_c = sha2_hash(&(poly_sx.eval(Mfp::from(6)) + poly_sx.eval(Mfp::from(7)) + Mfp::from(4)));

    let alpha = Mfp::from(10);
    let eta_a = Mfp::from(2);
    let eta_b = Mfp::from(30);
    let eta_c = Mfp::from(100);

    // Hardcoded values for test
    // let alpha = Mfp::from(54);
    // let eta_a = Mfp::from(123);
    // let eta_b = Mfp::from(92);
    // let eta_c = Mfp::from(47);

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
    // let mut points_row_p_a = get_matrix_point_row(&a_matrix, &set_h, &set_k);
    // let points_add = vec![
    //     (Mfp::from(48), Mfp::from(1)),
    //     (Mfp::from(73), Mfp::from(135)),
    //     (Mfp::from(62), Mfp::from(125)),
    //     (Mfp::from(132), Mfp::from(59)),
    //     (Mfp::from(65), Mfp::from(42)),
    //     (Mfp::from(80), Mfp::from(1)),
    // ];

    // Rand moka
    let mut points_row_p_a = HashMap::new();
    let points_add = vec![
        (Mfp::from(1), Mfp::from(609564788)),
        (Mfp::from(1536266199), Mfp::from(1956349769)),
        (Mfp::from(109072139), Mfp::from(645581151)),
        (Mfp::from(1663994522), Mfp::from(1956349769)),
        (Mfp::from(923639751), Mfp::from(645581151)),
        (Mfp::from(191227677), Mfp::from(1)),
        (Mfp::from(1684585140), Mfp::from(815036133)),
        (Mfp::from(1670695976), Mfp::from(1956349769)),
        (Mfp::from(380434607), Mfp::from(609564788)),
    ];
    for (k, v) in points_add {
        points_row_p_a.insert(k, v);
    }



    // let mut points_col_p_a = get_matrix_point_col(&a_matrix, &set_h, &set_k);
    // let points_add = vec![
    //     (Mfp::from(48), Mfp::from(42)),
    //     (Mfp::from(73), Mfp::from(1)),
    //     (Mfp::from(62), Mfp::from(135)),
    //     (Mfp::from(132), Mfp::from(125)),
    //     (Mfp::from(65), Mfp::from(59)),
    //     (Mfp::from(80), Mfp::from(42)),
    // ];
    // for (k, v) in points_add {
    //     points_col_p_a.insert(k, v);
    // }

    // Moka
    let mut points_col_p_a = HashMap::new();
    let points_add = vec![
        (Mfp::from(1), Mfp::from(815036133)),
        (Mfp::from(1536266199), Mfp::from(1)),
        (Mfp::from(109072139), Mfp::from(1956349769)),
        (Mfp::from(1663994522), Mfp::from(1956349769)),
        (Mfp::from(923639751), Mfp::from(645581151)),
        (Mfp::from(191227677), Mfp::from(1)),
        (Mfp::from(1684585140), Mfp::from(815036133)),
        (Mfp::from(1670695976), Mfp::from(609564788)),
        (Mfp::from(380434607), Mfp::from(1956349769)),
    ];
    for (k, v) in points_add {
        points_col_p_a.insert(k, v);
    }


    let points_val_p_a = get_matrix_point_val(&a_matrix, &set_h, &set_k, &points_row_p_a, &points_col_p_a);
    // println!("{:?}", points_val_p_a);

    // Matrix B: 
    // let mut points_row_p_b = get_matrix_point_row(&b_matrix, &set_h, &set_k);
    // let points_add = vec![
    //     (Mfp::from(73), Mfp::from(59)),
    //     (Mfp::from(62), Mfp::from(1)),
    //     (Mfp::from(132), Mfp::from(42)),
    //     (Mfp::from(65), Mfp::from(135)),
    //     (Mfp::from(80), Mfp::from(59)),
    // ];
    // for (k, v) in points_add {
    //     points_row_p_b.insert(k, v);
    // }


    // Moka
    let mut points_row_p_b = HashMap::new();
    let points_add = vec![
        (Mfp::from(1), Mfp::from(609564788)),
        (Mfp::from(1536266199), Mfp::from(1956349769)),
        (Mfp::from(109072139), Mfp::from(1956349769)),
        (Mfp::from(1663994522), Mfp::from(645581151)),
        (Mfp::from(923639751), Mfp::from(645581151)),
        (Mfp::from(191227677), Mfp::from(1)),
        (Mfp::from(1684585140), Mfp::from(815036133)),
        (Mfp::from(1670695976), Mfp::from(609564788)),
        (Mfp::from(380434607), Mfp::from(1956349769)),
    ];
    for (k, v) in points_add {
        points_row_p_b.insert(k, v);
    }

    // let mut points_col_p_b = get_matrix_point_col(&b_matrix, &set_h, &set_k);
    // let points_add = vec![
    //     (Mfp::from(73), Mfp::from(59)),
    //     (Mfp::from(62), Mfp::from(42)),
    //     (Mfp::from(132), Mfp::from(125)),
    //     (Mfp::from(65), Mfp::from(1)),
    //     (Mfp::from(80), Mfp::from(135)),
    // ];
    // for (k, v) in points_add {
    //     points_col_p_b.insert(k, v);
    // }

    // Moka
    let mut points_col_p_b = HashMap::new();
    let points_add = vec![
        (Mfp::from(1), Mfp::from(1)),
        (Mfp::from(1536266199), Mfp::from(1)),
        (Mfp::from(109072139), Mfp::from(609564788)),
        (Mfp::from(1663994522), Mfp::from(1)),
        (Mfp::from(923639751), Mfp::from(645581151)),
        (Mfp::from(191227677), Mfp::from(1)),
        (Mfp::from(1684585140), Mfp::from(815036133)),
        (Mfp::from(1670695976), Mfp::from(609564788)),
        (Mfp::from(380434607), Mfp::from(1956349769)),
    ];
    for (k, v) in points_add {
        points_col_p_b.insert(k, v);
    }
    

    let points_val_p_b = get_matrix_point_val(&b_matrix, &set_h, &set_k, &points_row_p_b, &points_col_p_b);
    // println!("{:?}", points_val_p_a);


    // // Matrix C: 
    // let mut points_row_p_c = get_matrix_point_row(&c_matrix, &set_h, &set_k);
    // let points_add = vec![
    //     (Mfp::from(48), Mfp::from(1)),
    //     (Mfp::from(73), Mfp::from(59)),
    //     (Mfp::from(62), Mfp::from(125)),
    //     (Mfp::from(132), Mfp::from(1)),
    //     (Mfp::from(65), Mfp::from(135)),
    //     (Mfp::from(80), Mfp::from(42)),
    // ];
    // for (k, v) in points_add {
    //     points_row_p_c.insert(k, v);
    // }

    // Moka
    let mut points_row_p_c = HashMap::new();
    let points_add = vec![
        (Mfp::from(1), Mfp::from(609564788)),
        (Mfp::from(1536266199), Mfp::from(1956349769)),
        (Mfp::from(109072139), Mfp::from(645581151)),
        (Mfp::from(1663994522), Mfp::from(1956349769)),
        (Mfp::from(923639751), Mfp::from(645581151)),
        (Mfp::from(191227677), Mfp::from(1)),
        (Mfp::from(1684585140), Mfp::from(815036133)),
        (Mfp::from(1670695976), Mfp::from(609564788)),
        (Mfp::from(380434607), Mfp::from(1956349769)),
    ];
    for (k, v) in points_add {
        points_row_p_c.insert(k, v);
    }
    

    // let mut points_col_p_c = get_matrix_point_col(&c_matrix, &set_h, &set_k);
    // let points_add = vec![
    //     (Mfp::from(48), Mfp::from(125)),
    //     (Mfp::from(73), Mfp::from(59)),
    //     (Mfp::from(62), Mfp::from(1)),
    //     (Mfp::from(132), Mfp::from(1)),
    //     (Mfp::from(65), Mfp::from(42)),
    //     (Mfp::from(80), Mfp::from(59)),
    // ];
    // for (k, v) in points_add {
    //     points_col_p_c.insert(k, v);
    // }

    // Moka
    let mut points_col_p_c = get_matrix_point_col(&c_matrix, &set_h, &set_k);
    let points_add = vec![
        (Mfp::from(1), Mfp::from(609564788)),
        (Mfp::from(1536266199), Mfp::from(1956349769)),
        (Mfp::from(109072139), Mfp::from(645581151)),
        (Mfp::from(1663994522), Mfp::from(1956349769)),
        (Mfp::from(923639751), Mfp::from(645581151)),
        (Mfp::from(191227677), Mfp::from(1)),
        (Mfp::from(1684585140), Mfp::from(815036133)),
        (Mfp::from(1670695976), Mfp::from(609564788)),
        (Mfp::from(380434607), Mfp::from(1956349769)),
    ];
    for (k, v) in points_add {
        points_col_p_c.insert(k, v);
    }

    let points_val_p_c = get_matrix_point_val(&c_matrix, &set_h, &set_k, &points_row_p_c, &points_col_p_c);
    // println!("{:?}", points_val_p_a);


    // ∑ r(alpha=10, k) * A^(k,x)
    let r_a_kx = sigma_rkx_mkx(&set_h, alpha, &points_val_p_a, &points_row_p_a, &points_col_p_a);
    println!("Poly ∑ r(alpha=10, k) * A^(k,x): A_h");
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
    // let beta_1 = gen_rand_not_in_set(&vec_to_set(&set_h));
    // let beta_1 = Mfp::from(sha2_hash(&poly_sx.eval(Mfp::from(9)).to_string()));
    let beta_1 = Mfp::from(22);

    // ∑ r(alpha=10, k) * A^(x,k)
    let r_a_xk = m_xk(&beta_1, &points_val_p_a, &points_row_p_a, &points_col_p_a, set_h.len());
    println!("Poly ∑ r(alpha=10, k) * A^(x,k): ");
    dsp_poly!(r_a_xk);

    // ∑ r(alpha=10, k) * B^(x,k)
    let r_b_xk = m_xk(&beta_1, &points_val_p_b, &points_row_p_b, &points_col_p_b, set_h.len());
    println!("Poly ∑ r(alpha=10, k) * B^(x,k): ");
    dsp_poly!(r_b_xk);

    // ∑ r(alpha=10, k) * C^(x,k)
    let r_c_xk = m_xk(&beta_1, &points_val_p_c, &points_row_p_c, &points_col_p_c, set_h.len());
    println!("Poly ∑ r(alpha=10, k) * C^(x,k): ");
    dsp_poly!(r_c_xk);
    

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
    // let beta_2 = gen_rand_not_in_set(&vec_to_set(&set_h));
    // let beta_2 = Mfp::from(sha2_hash(&poly_sx.eval(Mfp::from(10)).to_string()));
    let beta_2 = Mfp::from(80);

    println!("-------------------------------------------------");
    let a_row_px = sigma_yi_li(&points_row_p_a, &set_k);
    println!("a_row_px: ");
    dsp_poly!(a_row_px);
    println!("-------------------------------------------------");

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

    let concat_str = format!("{}{}{}{}{}", "zkIoT", "MultiSensor", "1.0", "1.0", "");
    store_commit_json(&[&a_row_px, &a_col_px, &a_val_px, &b_row_px, &b_col_px, &b_val_px, &c_row_px, &c_col_px, &c_val_px], m as usize, n as usize)?;
    
    // let commit_vals = restore_commit_json("commit.json")?;
    // let a_row_px = commit_vals.0[0].clone();
    // let a_col_px = commit_vals.0[1].clone();
    // let a_val_px = commit_vals.0[2].clone();
    // let b_row_px = commit_vals.0[3].clone();
    // let b_col_px = commit_vals.0[4].clone();
    // let b_val_px = commit_vals.0[5].clone();
    // let c_row_px = commit_vals.0[6].clone();
    // let c_col_px = commit_vals.0[7].clone();
    // let c_val_px = commit_vals.0[8].clone();
    // let m = commit_vals.1;
    // let n = commit_vals.2;


    // sigma_3 
    let mut sigma_3 = Mfp::ZERO; 

    // f_3x 
    let mut points_f_3: Vec<Point> = vec![];  
    for k in set_k.iter() {
        let sig_a = sigma_m(&van_poly_vhx, &eta_a, &beta_1, &beta_2, k, &[&a_row_px, &a_col_px, &a_val_px]);
        let sig_b = sigma_m(&van_poly_vhx, &eta_b, &beta_1, &beta_2, k, &[&b_row_px, &b_col_px, &b_val_px]);
        let sig_c = sigma_m(&van_poly_vhx, &eta_c, &beta_1, &beta_2, k, &[&c_row_px, &c_col_px, &c_val_px]);

        let sum = sig_a + sig_b + sig_c;
        sigma_3 += sum;
        points_f_3.push((*k, sum));
    }
    println!("sigma_3: {}", sigma_3);

    // a(x) 
    let poly_pi_a = (Poly::from(vec![beta_2]) -  &a_row_px) * (Poly::from(vec![beta_1]) -  &a_col_px);
    let poly_pi_b = (Poly::from(vec![beta_2]) -  &b_row_px) * (Poly::from(vec![beta_1]) -  &b_col_px);
    let poly_pi_c = (Poly::from(vec![beta_2]) -  &c_row_px) * (Poly::from(vec![beta_1]) -  &c_col_px);

    println!("================");
    dsp_poly!(poly_pi_a);
    dsp_poly!(poly_pi_b);
    dsp_poly!(poly_pi_b);
    println!("================");

    let poly_sig_a = Poly::from(vec![eta_a * van_poly_vhx.eval(beta_2) * van_poly_vhx.eval(beta_1)]) * &a_val_px;
    let poly_sig_b = Poly::from(vec![eta_b * van_poly_vhx.eval(beta_2) * van_poly_vhx.eval(beta_1)]) * &b_val_px;
    let poly_sig_c = Poly::from(vec![eta_c * van_poly_vhx.eval(beta_2) * van_poly_vhx.eval(beta_1)]) * &c_val_px;

    println!("================");
    dsp_poly!(poly_sig_a);
    dsp_poly!(poly_sig_b);
    dsp_poly!(poly_sig_c);
    println!("================");

    let poly_a_x = poly_sig_a * (&poly_pi_b * &poly_pi_c) + 
                   poly_sig_b * (&poly_pi_a * &poly_pi_c) +
                   poly_sig_c * (&poly_pi_a * &poly_pi_b);

    println!("a(x): {}", poly_a_x.eval(Mfp::from(5)));
    dsp_poly!(poly_a_x);
    
    // b(x)
    let poly_b_x = &poly_pi_a * &poly_pi_b * &poly_pi_c; 

    println!("b(x): {}", poly_b_x.eval(Mfp::from(5)));
    dsp_poly!(poly_b_x);

    let van_poly_vkx = vanishing_poly(&set_k);

    println!("van_poly_vkx");
    dsp_poly!(van_poly_vkx);

    let poly_f_3x = lagrange_interpolate(&points_f_3);

    let sigma_3_set_k = Mfp::from(sigma_3 / Mfp::from(set_k.len() as u64));
    println!("sigma_3_set_k {}", sigma_3_set_k);

    let poly_f_3x = poly_f_3x - Poly::from(vec![sigma_3_set_k]);

    println!("poly_f_3x");
    dsp_poly!(poly_f_3x);

    let g_3x = poly_f_3x.div_mod(&Poly::from(vec![Mfp::ONE, Mfp::ZERO])).0;
    
    println!("g_3x");
    dsp_poly!(g_3x);

    println!("1: ");
    let a = (poly_f_3x.clone() + Poly::from(vec![sigma_3_set_k]));
    dsp_poly!(a);

    println!("2: ");
    let a = (&poly_b_x * (poly_f_3x.clone() + Poly::from(vec![sigma_3_set_k])));
    dsp_poly!(a);


    println!("3: ");
    let a = (poly_a_x.clone() - (&poly_b_x * (poly_f_3x.clone() + Poly::from(vec![sigma_3_set_k]))));
    dsp_poly!(a);


    let h_3x = (poly_a_x.clone() - (&poly_b_x * (poly_f_3x.clone() + Poly::from(vec![sigma_3_set_k])))).div_mod(&van_poly_vkx).0;


    println!("h_3x");
    dsp_poly!(h_3x);


    store_proof_json(
        &[
            &poly_w_hat,
            &poly_z_hat_a,
            &poly_z_hat_b,
            &poly_z_hat_c,
            &poly_z_hat_x,
            &poly_h_0,
            &poly_sx,
            &g_1x,
            &h_1x,
            &g_2x,
            &h_2x,
            &g_3x,
            &h_3x
        ],
        &[&sigma_1, &sigma_2, &sigma_3],
        b, 
        set_h.len(),
        set_k.len()
    )?;


    // let proof_vals = restore_proof_json("proof.json")?;
    // let poly_h_0 = proof_vals.0[4].clone();
    // let poly_sx = proof_vals.0[5].clone();
    // let g_1x = proof_vals.0[6].clone();
    // let h_1x = proof_vals.0[7].clone();
    // let g_2x = proof_vals.0[8].clone();
    // let h_2x = proof_vals.0[9].clone();
    // let g_3x = proof_vals.0[10].clone();
    // let h_3x = proof_vals.0[11].clone();
    
    // let beta_3 =  Mfp::from(thread_rng().gen_range(1..(P - set_h.len() as u64)));
    let beta_3 = Mfp::from(5);

    let verify_res = verify(
        &h_1x,
        &g_1x,
        &h_2x,
        &g_2x,
        &h_3x,
        &g_3x,

        &beta_1,
        &sigma_1,
        &beta_2,
        &sigma_2,
        &beta_3,
        &sigma_3,

        &poly_a_x, //
        &poly_b_x, //
        &poly_ab_c,
        &poly_h_0,
        &poly_r,
        &poly_sx,
        &poly_z_hat_x, //

        set_k.len(),
        set_h.len(),
        
        &sum_1, //
        &van_poly_vkx, // 
        &van_poly_vhx, //
    );
    
    println!("Verify result: {}", verify_res);

    println!("time: {:?}", timer.elapsed());

    Ok(())
}
