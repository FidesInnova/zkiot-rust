use ark_ff::{Field, PrimeField};
use nalgebra::{Const, DMatrix, DVector};
use parser::parse_from_lines;
use rustnomial::{Evaluable, Polynomial};
use std::{path::PathBuf, u64};
use zk_iot::*;

// field finit parameter
const P: u64 = 181;

fn main() {
    println!("setup: -------------------------------------------------------------------");
    // Setup ================================================================================
    // init ----------------------------------
    // the number of gates
    let ng = 3;
    // the number of outputs
    let no = 1;
    // the number of inputs or register number 
    let ni = 1;

    // t rows of the matrices are set to zero 
    // or we can use for divie set_H in AHP
    // Actully: |x| = t
    let t = ni + 1;

    // generator number
    let g = 2;

    // d is a big u64 int
    let d = 111213119_u64;
    // according to the example, we should have this seq: 2, 2^119, 2^(119^2), ..., so we should find the exp-base which is 119.
    let l: u64 = 8;

    // matrix size
    let size = ng + ni + 1;

    // in the matrices, the cells are elements of the finite field P
    let mut a_matrix = DMatrix::<Mfp>::zeros(size, size);
    let mut b_matrix = DMatrix::<Mfp>::zeros(size, size);
    let mut c_matrix = DMatrix::<Mfp>::zeros(size, size);

    let mut z_poly = DVector::<Mfp>::zeros(size);
    z_poly[0] = Mfp::ONE;

    // R1(1)​−4=0                            => R1(1) = 4
    let r1 = Mfp::from(4);
    z_poly[1] = r1;

    // let gates = parser(PathBuf::from("sample.txt")).unwrap();

    let gates = parse_from_lines(&PathBuf::from("line_num.txt"), &PathBuf::from("sample.txt")).unwrap();
    // ---------------------------------------

    // A, B, C, z
    init(
        gates,
        ni,
        &mut a_matrix,
        &mut b_matrix,
        &mut c_matrix,
        &mut z_poly,
    );

    rows_to_zero(&mut a_matrix, t);
    rows_to_zero(&mut b_matrix, t);
    rows_to_zero(&mut c_matrix, t);
    let cz = (&a_matrix * &z_poly).component_mul(&(&b_matrix * &z_poly));

    println!("A:");
    mat_dsp!(&a_matrix);

    println!("B:");
    mat_dsp!(&b_matrix);

    println!("C:");
    mat_dsp!(&c_matrix);

    println!("Cz:");
    mat_dsp!(cz);

    // calculate proof path
    let mut pp = vec![];

    let mut s = Mfp::from(g);
    let d = d % (P - 1);

    for _ in 0..=l {
        pp.push(s);
        s = exp_mod(to_bint!(s), d);
    }

    println!();
    println!("proof path:\t( {} )", vec_dsp!(pp));
    // Commit ===============================================================================
    println!("\ncommit: ------------------------------------------------------------------");
    let n = 5;
    let m = 9;

    let generator_h = to_bint!(exp_mod(g, (P - 1) / n));
    let generator_k = to_bint!(exp_mod(g, (P - 1) / m));

    let set_h = generate_set(generator_h, n);
    let set_k = generate_set(generator_k, m);

    println!("H:\t{{ {} }}\nK:\t{{ {} }}", vec_dsp!(set_h), vec_dsp!(set_k));

    // A matrix --------------------------------------
    println!("A mat: =================================");
    let points = get_points_row(&a_matrix, &set_h, &set_k);
    let a_row = lagrange_interpolate(&points);
    println!("lag row:");
    poly_dsp!(a_row);

    let points = get_points_col(&a_matrix, &set_h, &set_k);
    let a_col = lagrange_interpolate(&points);
    println!("lag col:");
    poly_dsp!(a_col);
    
    let points = get_points_val(&a_matrix, &set_h, &set_h);
    let a_val = lagrange_interpolate(&points);
    println!("lag val:");
    poly_dsp!(a_val);

    let a_matrix_encode = vec![a_row, a_col, a_val];
    // ---------------------------------------

    // B matrix --------------------------------------
    println!("B mat: =================================");
    let points = get_points_row(&b_matrix, &set_h, &set_k);
    let b_row = lagrange_interpolate(&points);
    println!("lag row:");
    poly_dsp!(b_row);

    let points = get_points_col(&b_matrix, &set_h, &set_k);
    let b_col = lagrange_interpolate(&points);
    println!("lag col:");
    poly_dsp!(b_col);

    let points = get_points_val(&b_matrix, &set_h, &set_h);
    let b_val = lagrange_interpolate(&points);
    println!("lag val:");
    poly_dsp!(b_val);

    let b_matrix_encode = vec![b_row, b_col, b_val];
    // ---------------------------------------

    // C matrix --------------------------------------
    println!("C mat: =================================");
    let points = get_points_row(&c_matrix, &set_h, &set_k);
    let c_row = lagrange_interpolate(&points);
    println!("lag row:");
    poly_dsp!(c_row);

    let points = get_points_col(&c_matrix, &set_h, &set_k);
    let c_col = lagrange_interpolate(&points);
    println!("lag col:");
    poly_dsp!(c_col);

    let points = get_points_val(&c_matrix, &set_h, &set_h);
    let c_val = lagrange_interpolate(&points);
    println!("lag val:");
    poly_dsp!(c_val);
    
    let c_matrix_encode = vec![c_row, c_col, c_val];
    // ---------------------------------------
    let mut o_i = vec![];

    // append the vectors
    o_i.extend(a_matrix_encode);
    o_i.extend(b_matrix_encode);
    o_i.extend(c_matrix_encode);

    let c = commit(o_i, d, g);
    println!("commit:\t( {} )", vec_dsp!(c));

    // Eval =================================================================================
    println!("\neval: --------------------------------------------------------------------");
    let seq_k = generate_set_eval(generator_h, n as usize, t, set_k.len());
    let points_h = get_points_set(&seq_k, &set_k);
    let lag_h = lagrange_interpolate(&points_h);

    println!("lag h:");
    poly_dsp!(lag_h);

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

    for k in set_k {
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

    let az: nalgebra::Matrix<Mfp, nalgebra::Dyn, Const<1>, nalgebra::VecStorage<Mfp, nalgebra::Dyn, Const<1>>> = &a_matrix * &z_poly;
    
    let bz = &b_matrix * &z_poly;
    let cz = &c_matrix * &z_poly;

    println!("Matrix Az:");
    mat_dsp!(az);

    println!("Matrix Bz");
    mat_dsp!(bz);

    println!("Matrix Cz");
    mat_dsp!(cz);

    let mut points_za = get_points_set(&mat_to_vec(&az), &set_h);
    let mut points_zb = get_points_set(&mat_to_vec(&bz), &set_h);
    let mut points_zc = get_points_set(&mat_to_vec(&cz), &set_h);

    let b = 2;
    // push_random_points(&mut points_za, b, &vec_to_hashset(&set_h));
    // push_random_points(&mut points_za, b, &vec_to_hashset(&set_h));
    // push_random_points(&mut points_za, b, &vec_to_hashset(&set_h));

    // random inertation for za: 
    points_za.push((Mfp::from(150), Mfp::from(5)));
    points_za.push((Mfp::from(80), Mfp::from(47)));

    // random inertation for zb: 
    points_zb.push((Mfp::from(150), Mfp::from(15)));
    points_zb.push((Mfp::from(80), Mfp::from(170)));

    // random inertation for zc: 
    points_zc.push((Mfp::from(150), Mfp::from(1)));
    points_zc.push((Mfp::from(80), Mfp::from(100)));

    
    let poly_za = lagrange_interpolate(&points_za);
    println!("^za(x):");
    poly_dsp!(poly_za);

    let poly_zb = lagrange_interpolate(&points_zb);
    println!("^zb(x):");
    poly_dsp!(poly_zb);


    let poly_zc = lagrange_interpolate(&points_zc);
    println!("^zc(x):");
    poly_dsp!(poly_zc);


    // H[>∣x∣]
    let set_h_1 = &set_h[0..t].to_vec();

    // H[<=∣x∣]
    let set_h_2 = &set_h[t..].to_vec();

    // x^(h):
    let z_vec = &mat_to_vec(&z_poly);
    let points = get_points_set(&z_vec[0..t].to_vec(), set_h_1);
    let poly_xh = lagrange_interpolate(&points);
    
    // w(h): 
    let z_vec = &mat_to_vec(&z_poly);
    let points = get_points_set(&z_vec[t..].to_vec(), set_h_2);
    let wh = lagrange_interpolate(&points);

    // v(h): vanishing polynomial V of set h1 or H[<=∣x∣]
    let van_poly_vh1 = vanishing_poly(set_h_1);

    let mut points_w = vec![];
    for i in set_h_2 {
        // wˉ(h): 
        let w_bar_h = (wh.eval(*i) - poly_xh.eval(*i)) * van_poly_vh1.eval(*i).inverse().unwrap();
        points_w.push((*i, w_bar_h));
    }
    // push_random_points(&mut points_w, b, &vec_to_hashset(&set_h));

    // insert random points for wˉ(h)
    // push_random_points(&mut points_w, b, &vec_to_hashset(&set_h));

    points_w.push((Mfp::from(150), Mfp::from(42)));
    points_w.push((Mfp::from(80), Mfp::from(180)));

    let poly_wh = lagrange_interpolate(&points_w);

    println!("w_hat:");
    poly_dsp!(poly_zc);

    // h_zero
    let vh_div = vanishing_poly(&set_h);
    let poly_hz = (&poly_za * &poly_zb - &poly_zc).div_mod(&vh_div);
    
    println!("h0(x):");
    poly_dsp!(poly_hz.0);

    // random polynomial
    // let poly_sx = poly_gen_randomly((2 * set_h.len()) + b - 1);
    let poly_sx = [5, 0, 101, 17, 0, 1, 20, 0, 0, 3, 115];
    let poly_sx = poly_sx.iter().map(|v| Mfp::from(*v)).collect::<Vec<Mfp>>();
    let poly_sx = Polynomial::from(poly_sx);
    
    let sig_1 = set_h.iter().fold(Mfp::ZERO, |acc, &v| acc + poly_sx.eval(v));
    println!("sig:\t{}", sig_1);
    
    // all genrate radomnly in F
    // let alpha = MFp::from(thread_rng().gen_range(0..P));
    // let eta_a = MFp::from(thread_rng().gen_range(0..P));
    // let eta_b = MFp::from(thread_rng().gen_range(0..P));
    // let eta_c = MFp::from(thread_rng().gen_range(0..P));

    let alpha = Mfp::from(10);
    let eta_a = Mfp::from(2);
    let eta_b = Mfp::from(30);
    let eta_c = Mfp::from(100);

    // Z^(x):
    let poly_z_hat_x = poly_wh * van_poly_vh1 + poly_xh; 
    
    // ∑ η​z​(x):
    let sigma_eta_z_x = Polynomial::new(vec![eta_a]) * &poly_za +
                        Polynomial::new(vec![eta_b]) * &poly_zb + 
                        Polynomial::new(vec![eta_c]) * &poly_zc;
    println!("sigma eta zx:");
    poly_dsp!(sigma_eta_z_x);

    // ∑ ηr(α,x): INCOMPLETE
    let sigma_eta_r = Polynomial::new(vec![eta_a])     +
                      Polynomial::new(vec![eta_b])     + 
                      Polynomial::new(vec![eta_c]);

    let poly_r = poly_div(alpha, set_h.len());
    println!("r:");
    poly_dsp!(poly_r);

    println!("r * sigma:");
    poly_dsp!((&poly_r * &sigma_eta_z_x));

    // Sum Check Protocol Formula:
    // TOTALLY INCOMPLETE
     let poly_scp = poly_sx 
                    + poly_r * sigma_eta_z_x 
                    - poly_z_hat_x * sigma_eta_r;


}
