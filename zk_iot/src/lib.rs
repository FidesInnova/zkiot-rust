//! incomplete ZKP scheme with "nalgebra" lib
pub mod parser;
pub mod utils;

extern crate nalgebra as na;
use ark_ff::{Field, PrimeField};
use na::{DMatrix, DVector};
use rand::{thread_rng, Rng};
use rustnomial::{Degree, FreeSizePolynomial, Polynomial, SizedPolynomial};
use utils::{Gate, GateType};
use std::collections::{HashMap, HashSet};
use std::ops::Neg;

pub const P: u64 = 181;
field!(Mfp, P);

pub type Poly = Polynomial<Mfp>;
pub type Poly2d = (Poly, Poly);

pub type Point = (Mfp, Mfp);
pub type Point2d = (Point, Mfp);

pub fn rows_to_zero(mat: &mut DMatrix<Mfp>, t: usize) {
    for i in 0..t {
        for j in 0..mat.ncols() {
            mat[(i, j)] = Mfp::ZERO;
        }
    }
}

pub fn init(
    gates: Vec<Gate>,
    ni: usize,
    a_mat: &mut DMatrix<Mfp>,
    b_mat: &mut DMatrix<Mfp>,
    c_mat: &mut DMatrix<Mfp>,
    z_poly: &mut DVector<Mfp>,
) {
    for (i, gate) in gates.iter().enumerate() {
        let index = 1 + ni + i;
        c_mat[(index, index)] = Mfp::ONE;

        let left_val = gate.val_left.map_or(Mfp::ONE, Mfp::from);
        let right_val = gate.val_right.map_or(Mfp::ONE, Mfp::from);

        match gate.gate_type {
            GateType::Add => {
                a_mat[(index, 0)] = Mfp::ONE;

                b_mat[(index, gate.inx_left)] = left_val;
                b_mat[(index, gate.inx_right)] = right_val;

                z_poly[i + 2] = z_poly[i + 1] + gate.val_right.map_or(Mfp::ZERO, Mfp::from);
            }
            GateType::Mul => {
                a_mat[(index, gate.inx_left)] = left_val;
                b_mat[(index, gate.inx_right)] = right_val;

                z_poly[i + 2] = z_poly[i + 1] * right_val;
            }
        }
    }
}

pub fn exp_mod(a: u64, b: u64) -> Mfp
{
    Mfp::from(a).pow(&[b])
}

pub fn poly_r_xy(alpha: Mfp, degree: usize) -> Poly {
    let mut numerator = Polynomial::new(vec![exp_mod(to_bint!(alpha), degree as u64)]);
    numerator.add_term(-Mfp::ONE, degree);

    let mut denominator = Polynomial::new(vec![alpha]);
    denominator.add_term(-Mfp::ONE, 1);

    numerator.div_mod(&denominator).0
}

pub fn lagrange_interpolate(points: &Vec<Point>) -> Poly {
    let mut poly_res: Poly = Polynomial::new(vec![Mfp::ZERO]);

    for (x_i, y_i) in points.iter() {
        let mut poly_nume_all: Poly = Polynomial::new(vec![Mfp::ONE]);
        let mut poly_deno_all = Mfp::ONE;
        for (x_j, _) in points.iter() {
            if x_i != x_j {
                let poly_nume: Poly =
                    Polynomial::new(vec![Mfp::ONE, Mfp::from(*x_j).neg()]);
                let poly_deno = Mfp::from(*x_i) - Mfp::from(*x_j);

                poly_nume_all *= poly_nume;
                poly_deno_all *= poly_deno;
            }
        }
        poly_res += Polynomial::<Mfp>::new(vec![*y_i])
            * (poly_nume_all * poly_deno_all.inverse().unwrap());
    }

    poly_res
}

pub fn generate_set(ms_gen: u64, len: u64) -> Vec<Mfp> {
    (0..len).map(|i| exp_mod(ms_gen, i)).collect()
}

pub fn commit(o: Vec<Poly>, d: u64, g: u64) -> Vec<Mfp> {
    let mut res = vec![];

    for poly in o {
        let mut res_poly = Mfp::ONE;

        if let Degree::Num(deg) = poly.degree() {
            let mut s = Mfp::from(g);
            let d = d % (P - 1);

            for i in 0..=deg {
                let coef = poly.terms[deg - i].into_bigint().0[0];
                let value = exp_mod(s.into_bigint().0[0], coef);
                res_poly *= value;
                s = exp_mod(s.into_bigint().0[0], d);
            }
        }

        if res_poly == Mfp::ONE {
            res.push(Mfp::from(g));
        } else {
            res.push(res_poly);
        }
    }

    res
}

define_get_points_fn!(get_points_row, row);
define_get_points_fn!(get_points_col, col);
define_get_points_fn!(get_points_val, val);


pub fn generate_set_eval(ms_gen: u64, n: usize, t: usize, len: usize) -> Vec<Mfp> {
    let mut set: Vec<Mfp> = vec![];
    for i in t..n {
        set.push(exp_mod(ms_gen, i as u64));
    }
    let zeros = len as isize - n as isize + t as isize;

    assert!(zeros >= 0);

    if zeros > 0 {
        for _ in 0..zeros {
            set.push(Mfp::ZERO);
        }
    }
    set
}

pub fn get_points_set(seq: &Vec<Mfp>, n: &Vec<Mfp>) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];

    assert!(seq.len() == n.len(), "sets are not equal");

    for point in n.iter().zip(seq.iter()) {
        points.push((*point.0, *point.1));
    }
    points
}


pub fn mat_to_vec(
    mat: &nalgebra::Matrix<
        Mfp,
        nalgebra::Dyn,
        na::Const<1>,
        nalgebra::VecStorage<Mfp, nalgebra::Dyn, na::Const<1>>,
    >,
) -> Vec<Mfp> {
    assert!(mat.ncols() < 2, "cannot convet to vec mat.ncols() < 2");

    let mut v: Vec<Mfp> = vec![];

    for i in 0..mat.nrows() {
        v.push(mat[(i, 0)]);
    }
    v
}

pub fn push_random_points(
    points: &mut Vec<Point>,
    b: u64,
    set_h: &HashSet<Mfp>,
) {
    let mut rng = thread_rng();
    for _ in 0..b {
        let d = gen_rand_not_in_set(set_h);
        let r = Mfp::from(rng.gen_range(0..P));
        points.push((d, r));
    }
}

pub fn vec_to_hashset(vec: &Vec<Mfp>) -> HashSet<Mfp> {
    vec.iter().cloned().collect()
}

pub fn gen_rand_not_in_set(set: &HashSet<Mfp>) -> Mfp {
    let mut rng = rand::thread_rng();
    let mut num;

    loop {
        num = Mfp::from(rng.gen_range(0..P));
        if !set.contains(&num) {
            break;
        }
    }
    num
}

pub fn vanishing_poly(set: &Vec<Mfp>) -> Poly {
    let mut vp = Polynomial::new(vec![Mfp::ONE]);

    for i in set {
        vp *= Polynomial::new(vec![Mfp::ONE, Mfp::ZERO]) - Polynomial::new(vec![*i]);
    }
    vp.trim();
    vp
}

pub fn poly_gen_randomly(deg: usize) -> Poly {
    let mut rng = rand::thread_rng();
    let mut poly = vec![];

    for _ in 0..deg {
        poly.push(Mfp::from(rng.gen_range(0..P)));
    }
    
    Polynomial::new(poly)
}


pub fn gen_2d_points(mat: &DMatrix<Mfp>, h: &Vec<Mfp>) ->  HashMap<Point, Mfp> {
    let mut res = HashMap::new();
    
    for i in 0..mat.nrows() {
        for j in 0..mat.ncols() {
            if mat[(i, j)] != Mfp::ZERO {
                res.insert((h[i], h[j]), mat[(i, j)]);
            }
        }
    }
    
    res
}

#[cfg(test)]
mod math_test {
    use super::*; 

    #[test] 
    fn test_exp_mod() {
        assert_eq!(exp_mod(2, 0), Mfp::ONE);
        assert_eq!(exp_mod(10, 0), Mfp::ONE);

        assert_eq!(exp_mod(2, 1), Mfp::from(2));
        assert_eq!(exp_mod(10, 1), Mfp::from(10));

        assert_eq!(exp_mod(2, 3), Mfp::from(8));  
        assert_eq!(exp_mod(3, 2), Mfp::from(9));  
        assert_eq!(exp_mod(5, 3), Mfp::from(125));

        assert_eq!(exp_mod(2, 10), Mfp::from(1024)); 
        assert_eq!(exp_mod(3, 5), Mfp::from(243));   

        assert_eq!(exp_mod(987654321987654321, 1234567890123456789), Mfp::from(42));
        assert_eq!(exp_mod(887654448019654120, 1139562969472691009), Mfp::from(55));
        assert_eq!(exp_mod(u64::MAX, 9223372036854775807), Mfp::from(65));
    }
}