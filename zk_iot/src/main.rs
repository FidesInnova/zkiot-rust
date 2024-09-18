use commitmnet::ahp;
// use rand::{thread_rng, Rng};
use rustnomial::{Evaluable, FreeSizePolynomial, Polynomial};
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
    let mut commitment = ahp::Commitment::new(&setup)?;
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

    let commit_res = commitment.commit_o(setup.long_const_val, setup.generator); // Generate the commitment
    println!("Commit_o:\t( {} )", dsp_vec!(commit_res)); // Display the commitment

    commitment.commit();

    // let concat_str = format!("{}{}{}{}{}", "zkIoT", "MultiSensor", "1.0", "1.0", "");
    // store_commit_json(&[&a_row_px, &a_col_px, &a_val_px, &b_row_px, &b_col_px, &b_val_px, &c_row_px, &c_col_px, &c_val_px], m as usize, n as usize)?;
    
    let commit_vals = restore_commit_json("commit.json")?;
    let a_row_px = commit_vals.0[0].clone();
    let a_col_px = commit_vals.0[1].clone();
    let a_val_px = commit_vals.0[2].clone();
    let b_row_px = commit_vals.0[3].clone();
    let b_col_px = commit_vals.0[4].clone();
    let b_val_px = commit_vals.0[5].clone();
    let c_row_px = commit_vals.0[6].clone();
    let c_col_px = commit_vals.0[7].clone();
    let c_val_px = commit_vals.0[8].clone();
    let poly_sx = commit_vals.0[9].clone();
    let set_h_len = commit_vals.1;
    let set_k_len = commit_vals.2;

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
    let beta_1 = Mfp::from(sha2_hash(&poly_sx.eval(Mfp::from(9)).to_string()));
    let beta_2 = Mfp::from(sha2_hash(&poly_sx.eval(Mfp::from(10)).to_string()));

    let mut van_poly_vhx = Poly::new(vec![]);
    van_poly_vhx.add_term(Mfp::ONE, set_h_len);
    van_poly_vhx.add_term(Mfp::from(-1), 0);

    let mut van_poly_vkx = Poly::new(vec![]);
    van_poly_vkx.add_term(Mfp::ONE, set_k_len);
    van_poly_vkx.add_term(Mfp::from(-1), 0);

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
    dsp_poly(&poly_pi_a);
    dsp_poly(&poly_pi_b);
    dsp_poly(&poly_pi_b);
    println!("================");

    let poly_sig_a = Poly::from(vec![eta_a * van_poly_vhx.eval(beta_2) * van_poly_vhx.eval(beta_1)]) * &a_val_px;
    let poly_sig_b = Poly::from(vec![eta_b * van_poly_vhx.eval(beta_2) * van_poly_vhx.eval(beta_1)]) * &b_val_px;
    let poly_sig_c = Poly::from(vec![eta_c * van_poly_vhx.eval(beta_2) * van_poly_vhx.eval(beta_1)]) * &c_val_px;

    println!("================");
    dsp_poly(&poly_sig_a);
    dsp_poly(&poly_sig_b);
    dsp_poly(&poly_sig_c);
    println!("================");

    let poly_a_x = poly_sig_a * (&poly_pi_b * &poly_pi_c) + 
                   poly_sig_b * (&poly_pi_a * &poly_pi_c) +
                   poly_sig_c * (&poly_pi_a * &poly_pi_b);

    println!("a(x): {}", poly_a_x.eval(Mfp::from(5)));
    dsp_poly(&poly_a_x);
    
    // b(x)
    let poly_b_x = &poly_pi_a * &poly_pi_b * &poly_pi_c; 

    println!("b(x): {}", poly_b_x.eval(Mfp::from(5)));
    dsp_poly(&poly_b_x);

    println!("van_poly_vkx");
    dsp_poly(&van_poly_vkx);

    let poly_f_3x = lagrange_interpolate(&points_f_3);

    let sigma_3_set_k = Mfp::from(sigma_3 / Mfp::from(set_k_len as u64));
    println!("sigma_3_set_k {}", sigma_3_set_k);

    let poly_f_3x = poly_f_3x - Poly::from(vec![sigma_3_set_k]);

    println!("poly_f_3x");
    dsp_poly(&poly_f_3x);

    let g_3x = poly_f_3x.div_mod(&Poly::from(vec![Mfp::ONE, Mfp::ZERO])).0;
    
    println!("g_3x");
    dsp_poly(&g_3x);

    println!("1: ");
    let a = (poly_f_3x.clone() + Poly::from(vec![sigma_3_set_k]));
    dsp_poly(&a);

    println!("2: ");
    let a = (&poly_b_x * (poly_f_3x.clone() + Poly::from(vec![sigma_3_set_k])));
    dsp_poly(&a);


    println!("3: ");
    let a = (poly_a_x.clone() - (&poly_b_x * (poly_f_3x.clone() + Poly::from(vec![sigma_3_set_k]))));
    dsp_poly(&a);


    let h_3x = (poly_a_x.clone() - (&poly_b_x * (poly_f_3x.clone() + Poly::from(vec![sigma_3_set_k])))).div_mod(&van_poly_vkx).0;


    println!("h_3x");
    dsp_poly(&h_3x);


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


    // // let proof_vals = restore_proof_json("proof.json")?;
    // // let poly_h_0 = proof_vals.0[4].clone();
    // // let poly_sx = proof_vals.0[5].clone();
    // // let g_1x = proof_vals.0[6].clone();
    // // let h_1x = proof_vals.0[7].clone();
    // // let g_2x = proof_vals.0[8].clone();
    // // let h_2x = proof_vals.0[9].clone();
    // // let g_3x = proof_vals.0[10].clone();
    // // let h_3x = proof_vals.0[11].clone();
    
    // // let beta_3 =  Mfp::from(thread_rng().gen_range(1..(P - set_h.len() as u64)));
    // let beta_3 = Mfp::from(5);

    // let verify_res = verify(
    //     &h_1x,
    //     &g_1x,
    //     &h_2x,
    //     &g_2x,
    //     &h_3x,
    //     &g_3x,

    //     &beta_1,
    //     &sigma_1,
    //     &beta_2,
    //     &sigma_2,
    //     &beta_3,
    //     &sigma_3,

    //     &poly_a_x, //
    //     &poly_b_x, //
    //     &poly_ab_c,
    //     &poly_h_0,
    //     &poly_r,
    //     &poly_sx,
    //     &poly_z_hat_x, //

    //     set_k.len(),
    //     set_h.len(),
        
    //     &sum_1, //
    //     &van_poly_vkx, // 
    //     &van_poly_vhx, //
    // );
    
    // println!("Verify result: {}", verify_res);

    // println!("time: {:?}", timer.elapsed());

    Ok(())
}
