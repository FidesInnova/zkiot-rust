use ark_ff::{Field, PrimeField};
use nalgebra::{Const, DMatrix, DVector, SMatrix, U1};
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
    // the number of inputs
    let ni = 1;
    // matrix size
    let size = ng + ni + 1;

    // t rows of the matrices are set to zero
    let t = ni + 1;

    // gate number
    let g = 2;

    // d is a big u64 int
    let d = 111213119_u64;
    // according to the example, we should have this seq: 2, 2^119, 2^(119^2), ..., so we should find the exp-base which is 119.
    let l: u64 = 8;

    // in the matrices, the cells are elements of the finite field P
    let mut a_matrix = DMatrix::<MFp<P>>::zeros(size, size);
    let mut b_matrix = DMatrix::<MFp<P>>::zeros(size, size);
    let mut c_matrix = DMatrix::<MFp<P>>::zeros(size, size);

    let mut z_poly = DVector::<MFp<P>>::zeros(size);
    z_poly[0] = MFp::ONE;

    // R1(1)​−4=0                            => R1(1) = 4
    let r1 = MFp::<P>::from(4);
    z_poly[1] = r1;
    // gates according to wiki example
    // R1(2)−5R1(1)=0R1(2)​−5R1(1)​=0         => R1(2) = R1(1) * 5
    // R1(3)−R1(2)−11=0R1(3)​−R1(2)​−11=0     => R1(3) = R1(2) + 11
    // R1(4)−R1(3)/7=0                      => R1(4) = R1(3) * 1/7

    let gates = parser(PathBuf::from("sample.txt")).unwrap();
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

    zero_t_rows(&mut a_matrix, t);
    zero_t_rows(&mut b_matrix, t);
    zero_t_rows(&mut c_matrix, t);
    let cz = (&a_matrix * &z_poly).component_mul(&(&b_matrix * &z_poly));

    println!("Matrix A:");
    print_mat(&a_matrix);

    println!("Matrix B:");
    print_mat(&b_matrix);

    println!("Matrix C:");
    print_mat(&c_matrix);

    println!("cz=\n{}", cz);

    // calculate proof path
    let mut pp = vec![];

    let mut s = MFp::<P>::from(g);
    let d = d % (P - 1);

    for _ in 0..=l {
        pp.push(s);
        s = exp_mod::<P>(s.into_bigint().0[0], d);
    }

    println!();
    println!("proof path: {:?}", pp);
    // Commit ===============================================================================
    println!("commit: ------------------------------------------------------------------");
    let n = 5;
    let m = 9;

    let generator_h = exp_mod::<P>(g, (P - 1) / n).into_bigint().0[0];
    let generator_k = exp_mod::<P>(g, (P - 1) / m).into_bigint().0[0];

    let set_h = generate_set::<P>(generator_h, n);
    let set_k = generate_set::<P>(generator_k, m);

    println!("H= {:?}\nK= {:?}", set_h, set_k);

    // A matrix --------------------------------------
    println!("A mat: =================================");
    let points = get_poinsts_row(&a_matrix, &set_h, &set_k);
    let a_row = lagrange_interpolate::<P>(&points);
    println!("lag row: {:?}", a_row);

    let points = get_poinsts_col(&a_matrix, &set_h, &set_k);
    let a_col = lagrange_interpolate::<P>(&points);
    println!("lag col: {:?}", a_col);

    let points = get_poinsts_val(&a_matrix, &set_h);
    let a_val = lagrange_interpolate::<P>(&points);
    println!("lag val: {:?}", a_val);

    let a_matrix_encode = vec![a_row, a_col, a_val];
    // ---------------------------------------

    // B matrix --------------------------------------
    println!("B mat: =================================");
    let points = get_poinsts_row(&b_matrix, &set_h, &set_k);
    let b_row = lagrange_interpolate::<P>(&points);
    println!("lag row: {:?}", b_row);

    let points = get_poinsts_col(&b_matrix, &set_h, &set_k);
    let b_col = lagrange_interpolate::<P>(&points);
    println!("lag col: {:?}", b_col);

    let points = get_poinsts_val(&b_matrix, &set_h);
    let b_val = lagrange_interpolate::<P>(&points);
    println!("lag val: {:?}", b_val);

    let b_matrix_encode = vec![b_row, b_col, b_val];
    // ---------------------------------------

    // C matrix --------------------------------------
    println!("C mat: =================================");
    let points = get_poinsts_row(&c_matrix, &set_h, &set_k);
    let c_row = lagrange_interpolate::<P>(&points);
    println!("lag row: {:?}", c_row);

    let points = get_poinsts_col(&c_matrix, &set_h, &set_k);
    let c_col = lagrange_interpolate::<P>(&points);
    println!("lag col: {:?}", c_col);

    let points = get_poinsts_val(&c_matrix, &set_h);
    let c_val = lagrange_interpolate::<P>(&points);
    println!("lag val: {:?}", c_val);

    let c_matrix_encode = vec![c_row, c_col, c_val];
    // ---------------------------------------
    let mut o_i = vec![];

    // append the vectors
    o_i.extend(a_matrix_encode);
    o_i.extend(b_matrix_encode);
    o_i.extend(c_matrix_encode);

    println!("O_i: {:?}", o_i);

    let c = commit(o_i, d, g);
    println!("commit: {:?}", c);

    // Eval =================================================================================
    println!("eval: --------------------------------------------------------------------");
    let seq_k = generate_set_eval::<P>(generator_h, n as usize, t, set_k.len());
    let points_h = get_points_set(&seq_k, &set_k);
    let lag_h = lagrange_interpolate::<P>(&points_h);

    println!("seq_k:\t\t{:?}", seq_k);
    println!("points_h:\t{:?}", points_h);
    println!("lag h:\t\t{:?}", lag_h);

    let a = [set_h[2], MFp::ZERO];

    let r = set_h[1];

    let c = [
        (n as isize - t as isize),
        m as isize - (n as isize - t as isize),
    ];

    let p = [0, 3];

    points_h.iter().find(|v| **v == (MFp::ONE, MFp::ONE));

    let res = a.iter().zip(p.iter()).all(|(a_i, p_i)| {
        let e = exp_mod::<P>(generator_k, *p_i);
        points_h.iter().any(|v| v.0 == e && v.1 == *a_i)
    });
    println!("Geo Seq Test result:\t{:?}", res);

    let mut res = false;

    for k in set_k {
        if let Some((_, h_gamma_k)) = points_h
            .iter()
            .find(|v| v.0 == k * MFp::<P>::from(generator_k))
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

    let seq_kh = vec![1, 56, 59, 46, 42];

    let az: nalgebra::Matrix<MFp<P>, nalgebra::Dyn, Const<1>, nalgebra::VecStorage<MFp<P>, nalgebra::Dyn, Const<1>>> = &a_matrix * &z_poly;
    
    let bz = &b_matrix * &z_poly;
    let cz = &c_matrix * &z_poly;

    println!("Matrix Az: {}", az);
    println!("Matrix Bz: {}", bz);
    println!("Matrix Cz: {}", cz);

    let points_za = get_points_set(&mat_to_vec::<P>(&az), &set_h);
    println!("{:?}", points_za);

    let points_zb = get_points_set(&mat_to_vec::<P>(&bz), &set_h);
    println!("{:?}", points_zb);

    let points_zc = get_points_set(&mat_to_vec::<P>(&cz), &set_h);
    println!("{:?}", points_zc);


    let points_za = get_points_set(&mat_to_vec::<P>(&az), &set_h);
    println!("{:?}", points_za);
    
    let za = lagrange_interpolate::<P>(&points_za);
    println!("za: {:?}", za);
}