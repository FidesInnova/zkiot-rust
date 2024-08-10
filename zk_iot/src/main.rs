// TODO: Check all random gens 

use ark_ff::{Field, PrimeField};
use nalgebra::{Const, DMatrix, DVector};
use parser::parse_from_lines;
use rand::thread_rng;
use rustnomial::{Evaluable, FreeSizePolynomial, Polynomial};
use std::{collections::HashMap, path::PathBuf, u64};
use zk_iot::*;

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
    dsp_mat!(&a_matrix);

    println!("B:");
    dsp_mat!(&b_matrix);

    println!("C:");
    dsp_mat!(&c_matrix);

    println!("Cz:");
    dsp_mat!(cz);

    // calculate proof path
    let mut pp = vec![];

    let mut s = Mfp::from(g);
    let d = d % (P - 1);

    for _ in 0..=l {
        pp.push(s);
        s = exp_mod(to_bint!(s), d);
    }

    println!();
    println!("proof path:\t( {} )", dsp_vec!(pp));
    // Commit ===============================================================================
    println!("\ncommit: ------------------------------------------------------------------");
    let n = 5;
    let m = 9;

    let generator_h = to_bint!(exp_mod(g, (P - 1) / n));
    let generator_k = to_bint!(exp_mod(g, (P - 1) / m));

    let set_h = generate_set(generator_h, n);
    let set_k = generate_set(generator_k, m);

    println!("H:\t{{ {} }}\nK:\t{{ {} }}", dsp_vec!(set_h), dsp_vec!(set_k));

    // A matrix --------------------------------------
    println!("A mat: =================================");
    let points = get_points_row(&a_matrix, &set_h, &set_k);
    let a_row = lagrange_interpolate(&points);
    println!("lag row:");
    dsp_poly!(a_row);

    let points = get_points_col(&a_matrix, &set_h, &set_k);
    let a_col = lagrange_interpolate(&points);
    println!("lag col:");
    dsp_poly!(a_col);
    
    let points = get_points_val(&a_matrix, &set_h, &set_h);
    let a_val = lagrange_interpolate(&points);
    println!("lag val:");
    dsp_poly!(a_val);

    let a_matrix_encode = vec![a_row, a_col, a_val];
    // ---------------------------------------

    // B matrix --------------------------------------
    println!("B mat: =================================");
    let points = get_points_row(&b_matrix, &set_h, &set_k);
    let b_row = lagrange_interpolate(&points);
    println!("lag row:");
    dsp_poly!(b_row);

    let points = get_points_col(&b_matrix, &set_h, &set_k);
    let b_col = lagrange_interpolate(&points);
    println!("lag col:");
    dsp_poly!(b_col);

    let points = get_points_val(&b_matrix, &set_h, &set_h);
    let b_val = lagrange_interpolate(&points);
    println!("lag val:");
    dsp_poly!(b_val);

    let b_matrix_encode = vec![b_row, b_col, b_val];
    // ---------------------------------------

    // C matrix --------------------------------------
    println!("C mat: =================================");
    let points = get_points_row(&c_matrix, &set_h, &set_k);
    let c_row = lagrange_interpolate(&points);
    println!("lag row:");
    dsp_poly!(c_row);

    let points = get_points_col(&c_matrix, &set_h, &set_k);
    let c_col = lagrange_interpolate(&points);
    println!("lag col:");
    dsp_poly!(c_col);

    let points = get_points_val(&c_matrix, &set_h, &set_h);
    let c_val = lagrange_interpolate(&points);
    println!("lag val:");
    dsp_poly!(c_val);
    
    let c_matrix_encode = vec![c_row, c_col, c_val];
    // ---------------------------------------
    let mut o_i = vec![];

    // append the vectors
    o_i.extend(a_matrix_encode);
    o_i.extend(b_matrix_encode);
    o_i.extend(c_matrix_encode);

    let c = commit(o_i, d, g);
    println!("commit:\t( {} )", dsp_vec!(c));

    // Eval =================================================================================
    println!("\neval: --------------------------------------------------------------------");
    let seq_k = generate_set_eval(generator_h, n as usize, t, set_k.len());
    let points_h = get_points_set(&seq_k, &set_k);
    let lag_h = lagrange_interpolate(&points_h);

    println!("lag h:");
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

    let az: nalgebra::Matrix<Mfp, nalgebra::Dyn, Const<1>, nalgebra::VecStorage<Mfp, nalgebra::Dyn, Const<1>>> = &a_matrix * &z_poly;
    
    let bz = &b_matrix * &z_poly;
    let cz = &c_matrix * &z_poly;

    println!("Matrix Az:");
    dsp_mat!(az);

    println!("Matrix Bz");
    dsp_mat!(bz);

    println!("Matrix Cz");
    dsp_mat!(cz);

    let mut points_za = get_points_set(&mat_to_vec(&az), &set_h);
    let mut points_zb = get_points_set(&mat_to_vec(&bz), &set_h);
    let mut points_zc = get_points_set(&mat_to_vec(&cz), &set_h);


    // TODO: b = random F(0-181) - H(...) / 0< b <= P-n
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
    dsp_poly!(poly_za);

    let poly_zb = lagrange_interpolate(&points_zb);
    println!("^zb(x):");
    dsp_poly!(poly_zb);


    let poly_zc = lagrange_interpolate(&points_zc);
    println!("^zc(x):");
    dsp_poly!(poly_zc);


    // H[>∣x∣]
    let set_h_1 = &set_h[0..t].to_vec();

    // H[<=∣x∣]
    let set_h_2 = &set_h[t..].to_vec();

    // x^(h):
    let z_vec = &mat_to_vec(&z_poly);
    let points = get_points_set(&z_vec[0..t].to_vec(), set_h_1);
    let poly_x_hat = lagrange_interpolate(&points);
    
    // w(h): 
    let z_vec = &mat_to_vec(&z_poly);
    let points = get_points_set(&z_vec[t..].to_vec(), set_h_2);
    let wh = lagrange_interpolate(&points);

    // v(h): vanishing polynomial V of set h1 or H[<=∣x∣]
    let van_poly_vh1 = vanishing_poly(set_h_1);

    let mut points_w = vec![];
    for i in set_h_2 {
        // wˉ(h): 
        let w_bar_h = (wh.eval(*i) - poly_x_hat.eval(*i)) * van_poly_vh1.eval(*i).inverse().unwrap();
        points_w.push((*i, w_bar_h));
    }
    // push_random_points(&mut points_w, b, &vec_to_hashset(&set_h));

    // insert random points for wˉ(h)
    // push_random_points(&mut points_w, b, &vec_to_hashset(&set_h));

    points_w.push((Mfp::from(150), Mfp::from(42)));
    points_w.push((Mfp::from(80), Mfp::from(180)));

    let poly_wh = lagrange_interpolate(&points_w);

    println!("w_hat:");
    dsp_poly!(poly_wh);

    // h_zero
    let vh_div = vanishing_poly(&set_h);
    let poly_hz = (&poly_za * &poly_zb - &poly_zc).div_mod(&vh_div);
    
    println!("h0(x):");
    dsp_poly!(poly_hz.0);

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

    
    // ∑ η​z​(x):
    let sigma_eta_z_x = Polynomial::new(vec![eta_a]) * &poly_za +
                        Polynomial::new(vec![eta_b]) * &poly_zb + 
                        Polynomial::new(vec![eta_c]) * &poly_zc;
    // println!("sigma eta zx:");
    // dsp_poly!(sigma_eta_z_x);

    
    let poly_r = poly_r_xy(alpha, set_h.len());
    // println!("r:");
    // dsp_poly!(poly_r);

    println!("r(α,x)∑M​ηM​z^M​(x):");
    dsp_poly!((&poly_r * &sigma_eta_z_x));

    

    // Z^(x):
    let poly_z_hat_x = poly_wh * van_poly_vh1 + poly_x_hat; 
    println!("z_hat: ");
    dsp_poly!(poly_z_hat_x);


    // Matrix A: 
    let mut points_row_p_a = get_matrix_point_row(&a_matrix, &set_h, &set_k);
    // let mut points_add = vec![
    //     (Mfp::from(48), Mfp::from(1)),
    //     (Mfp::from(73), Mfp::from(135)),
    //     (Mfp::from(62), Mfp::from(125)),
    //     (Mfp::from(132), Mfp::from(59)),
    //     (Mfp::from(65), Mfp::from(42)),
    //     (Mfp::from(80), Mfp::from(1)),
    // ];
    // points_row_p_a.append(&mut points_add);
    println!("{:?}", points_row_p_a);


    let mut points_col_p_a = get_matrix_point_col(&a_matrix, &set_h, &set_k);
    // let mut points_add = vec![
    //     (Mfp::from(48), Mfp::from(42)),
    //     (Mfp::from(73), Mfp::from(1)),
    //     (Mfp::from(62), Mfp::from(135)),
    //     (Mfp::from(132), Mfp::from(125)),
    //     (Mfp::from(65), Mfp::from(59)),
    //     (Mfp::from(80), Mfp::from(42)),
    // ];
    // points_col_p_a.append(&mut points_add);


    println!("{:?}", points_col_p_a);

    let u_poly_set_h = func_u(&set_h);



    let mut points_val_p_a = get_matrix_point_val(&a_matrix, &set_h, &set_k, &points_row_p_a, &points_col_p_a);
    println!("{:?}", points_val_p_a);
    

    // ∑ ηr(α,x): INCOMPLETE
    let sigma_eta_r = Polynomial::new(vec![eta_a])     +
                      Polynomial::new(vec![eta_b])     + 
                      Polynomial::new(vec![eta_c]);


    // Sum Check Protocol Formula:
    // TOTALLY INCOMPLETE
    let poly_scp = poly_sx // s(x) 
                    + poly_r * sigma_eta_z_x // r(a)*sigma 
                    - &poly_z_hat_x * sigma_eta_r; // sigma*z_hat
    
    // println!("scp: ");
    // dsp_poly!(poly_scp);
}

use rand::prelude::SliceRandom;

fn add_random_points(set: &mut Vec<(Mfp, Mfp)>, set_h: &Vec<Mfp>, set_k: &Vec<Mfp>, c: usize) {
    for i in c..set_k.len() {
        set.push((set_k[i], *set_h.choose(&mut thread_rng()).unwrap()));
    }
}

fn poly_m_xy(set_k: &Vec<Mfp>, alpha: Mfp, set_h_len: usize, row_p: Vec<Mfp>, col_p: Vec<Mfp>, val_p: Vec<Mfp>) -> Poly {
    let mut poly_res: Poly = Polynomial::new(vec![Mfp::ZERO]);
    let alpha_exp = exp_mod(to_bint!(alpha), set_h_len as u64);

    for &i in set_k {
        poly_r_xy(alpha, set_h_len); // ? 
    }

    todo!()
}

fn get_matrix_point_row(mat: &DMatrix<Mfp>, set_h: &Vec<Mfp>, set_k: &Vec<Mfp>) -> HashMap<Mfp, Mfp> {
    let mut res = HashMap::new();
    let mut t = 0;
    let mut c = 0;
    let mat_len = mat.nrows();

    'l: for i in 0..mat_len {
        for j in 0..mat_len {
            if mat[(i, j)] != Mfp::ZERO {
                t = i;
                break 'l;
            }
        }
    }

    for i in 0..mat_len {
        for j in 0..mat_len {
            if mat[(i, j)] != Mfp::ZERO {
                res.insert(set_k[c], set_h[t + c]);
                c += 1;
            }
        }
    }

    // TODO: uncomment it when you want choose randomly 
    // add_random_points(&mut res, set_h, set_k, c);
    
    res 
}

fn get_matrix_point_col(mat: &DMatrix<Mfp>, set_h: &Vec<Mfp>, set_k: &Vec<Mfp>) -> HashMap<Mfp, Mfp> {
    let mut res = HashMap::new();
    let mut t = 0;
    let mut c = 0;
    let mat_len = mat.nrows();

    'l: for i in 0..mat_len {
        for j in 0..mat_len {
            if mat[(j, i)] != Mfp::ZERO {
                t = i;
                break 'l;
            }
        }
    }

    for i in 0..mat_len {
        for j in 0..mat_len {
            if mat[(j, i)] != Mfp::ZERO {
                res.insert(set_k[c], set_h[t + c]);
                c += 1;
            }
        }
    }
    

    // TODO: uncomment it when you want choose randomly 
    // add_random_points(&mut res, set_h, set_k, c);

    res 
}


fn get_matrix_point_val(mat: &DMatrix<Mfp>, set_h: &Vec<Mfp>, set_k: &Vec<Mfp>, row_k: &HashMap<Mfp, Mfp>, col_k: &HashMap<Mfp, Mfp>) -> HashMap<Mfp, Mfp> {
    let mut res = HashMap::new();
    let mut c = 0;
    let mat_len = mat.nrows();

    for i in 0..mat_len {
        for j in 0..mat_len {
            if mat[(i, j)] != Mfp::ZERO {
                let val = mat[(i, j)];
                let poly_u = func_u(&set_h);
                let p2 =  val / ( poly_u.eval(row_k[&val]) * poly_u.eval(col_k[&val]) );
                res.insert(set_k[c], p2);
                c += 1;
            }
        }
    }
    
    res 
}



fn poly_r(val: Mfp, set_h_len: usize) -> Poly {
    let mut numerator = Polynomial::new(vec![-exp_mod(to_bint!(val), set_h_len as u64)]);
    numerator.add_term(Mfp::ONE, set_h_len);

    let mut denominator = Polynomial::new(vec![-val]);
    denominator.add_term(Mfp::ONE, 1);

    numerator.div_mod(&denominator).0
}

fn func_u(set: &Vec<Mfp>) -> Poly {
    let len = set.len();
    let mut poly = Poly::from(vec![Mfp::ZERO]);
    poly.add_term(Mfp::from(len as u64), len - 1);
    poly
}

// pub fn poly_r_xy(alpha: Mfp, degree: usize) -> Poly {
//     let mut numerator = Polynomial::new(vec![exp_mod(to_bint!(alpha), degree as u64)]);
//     numerator.add_term(-Mfp::ONE, degree);

//     let mut denominator = Polynomial::new(vec![alpha]);
//     denominator.add_term(-Mfp::ONE, 1);

//     numerator.div_mod(&denominator).0
// }
