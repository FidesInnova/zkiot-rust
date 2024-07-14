//! incomplete ZKP scheme with "nalgebra" lib

extern crate nalgebra as na;
use anyhow::{anyhow, Result};
use ark_ff::Fp64;
use ark_ff::{
    fields::{MontBackend, MontConfig},
    FpConfig,
};
use ark_ff::{Field, PrimeField};
use na::{DMatrix, DVector};
use rand::{thread_rng, Rng};
use rustnomial::{Degree, FreeSizePolynomial, Polynomial, SizedPolynomial};
use std::collections::HashSet;
use std::u64;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Neg,
    path::PathBuf,
};

const P: u64 = 181;

pub struct P64MontConfig<const N: u64>;
impl<const N: u64> MontConfig<1> for P64MontConfig<N> {
    const MODULUS: ark_ff::BigInt<1> = ark_ff::BigInt::new([N; 1]);
    const GENERATOR: ark_ff::Fp<MontBackend<Self, 1>, 1> = MontBackend::ONE;
    const TWO_ADIC_ROOT_OF_UNITY: ark_ff::Fp<MontBackend<Self, 1>, 1> =
        ark_ff::Fp::new(Self::MODULUS);
}

#[allow(warnings)]
pub type MFp<const N: u64> = Fp64<MontBackend<P64MontConfig<N>, 1>>;

#[derive(Debug, Clone, Copy)]
pub enum GateType {
    Add,
    Mul,
}

#[derive(Debug)]
pub struct Gate {
    pub inx_left: usize,
    pub inx_right: usize,
    pub val_left: Option<u64>,
    pub val_right: Option<u64>,
    pub gate_type: GateType,
}
impl Gate {
    pub fn new(
        l: usize,
        r: usize,
        val_left: Option<u64>,
        val_right: Option<u64>,
        gtype: GateType,
    ) -> Self {
        Self {
            inx_left: l,
            inx_right: r,
            val_left,
            val_right,
            gate_type: gtype,
        }
    }
}

pub fn zero_t_rows<const N: u64>(mat: &mut DMatrix<MFp<N>>, t: usize) {
    for i in 0..t {
        for j in 0..mat.ncols() {
            mat[(i, j)] = MFp::ZERO;
        }
    }
}

pub fn init<const N: u64>(
    gates: Vec<Gate>,
    ni: usize,
    a_mat: &mut DMatrix<MFp<N>>,
    b_mat: &mut DMatrix<MFp<N>>,
    c_mat: &mut DMatrix<MFp<N>>,
    z_poly: &mut DVector<MFp<N>>,
) {
    for (i, gate) in gates.iter().enumerate() {
        let index = 1 + ni + i;
        c_mat[(index, index)] = MFp::ONE;

        let left_val = gate.val_left.map_or(MFp::ONE, MFp::<N>::from);
        let right_val = gate.val_right.map_or(MFp::ONE, MFp::<N>::from);

        match gate.gate_type {
            GateType::Add => {
                a_mat[(index, 0)] = MFp::ONE;

                b_mat[(index, gate.inx_left)] = left_val;
                b_mat[(index, gate.inx_right)] = right_val;

                z_poly[i + 2] = z_poly[i + 1] + gate.val_right.map_or(MFp::ZERO, MFp::from);
            }
            GateType::Mul => {
                a_mat[(index, gate.inx_left)] = left_val;
                b_mat[(index, gate.inx_right)] = right_val;

                z_poly[i + 2] = z_poly[i + 1] * right_val;
            }
        }
    }
}

// this function calculates the modular exponentiaton of a base g raised to power of exp modulo N
pub fn exp_mod<const N: u64>(g: u64, exp: u64) -> MFp<N> {
    let mut res: MFp<N> = MFp::ONE;

    for _ in 0..exp {
        res *= MFp::from(g);
    }

    res
}


pub fn r_func<const N: u64>(alpha: MFp<N>, exp: usize) -> Polynomial<MFp<N>> {
    let mut de = Polynomial::new(vec![exp_mod(to_bint!(alpha), exp as u64)]);
    de.add_term(-MFp::<N>::ONE, exp);

    let mut fr = Polynomial::new(vec![alpha]);
    fr.add_term(-MFp::<N>::ONE, 1);

    de.div_mod(&fr).0
}

#[macro_export]
macro_rules! mat_dsp {
    ($mat: expr) => {
        for i in 0..$mat.nrows() {
            for j in 0..$mat.ncols() {
                let derr = $mat[(i, j)];
                print!(
                    "{}\t",
                    if derr == MFp::ZERO {
                        "0".to_owned()
                    } else {
                        format!("{}", derr)
                    }
                );
            }
            println!();
        }
        println!();
    };
}

#[macro_export]
macro_rules! vec_dsp {
    ($ve: expr) => {
        $ve.iter()
            .fold(String::new(), |acc, x| acc + &format!("{}, ", *x))
    };
}

#[macro_export]
macro_rules! to_bint {
    ($var: expr) => {
        ($var).into_bigint().0[0]
    };
}

#[macro_export]
macro_rules! poly_dsp {
    ($poly:expr) => {{
        let mut result = String::new();
        for (i, term) in $poly.terms.iter().enumerate() {
            if i > 0 {
                result.push_str(" + ");
            }
            result.push_str(&format!("{}x^{}", term, i));
        }
        result
    }};
}

pub fn print_poly<const N: u64>(p: &Polynomial<MFp<N>>) -> String {
    let mut result = String::new();
    if let Degree::Num(deg) = p.degree() {
        for (i, term) in p.terms.iter().enumerate() {
            if *term != MFp::ZERO {
                if i != 0 {
                    result.push_str(" + ");
                }
                if *term == MFp::ONE {
                    result.push_str(&format!("x^{}", deg - i));
                } else if deg - i == 0 {
                    result.push_str(&format!("{}", term));
                } else if deg - i == 1 {
                    result.push_str(&format!("{}x", term));
                } else {
                    result.push_str(&format!("{}x^{}", term, deg - i));
                }
            }
        }
    }

    result
}

pub fn lagrange_interpolate<const N: u64>(points: &Vec<(MFp<N>, MFp<N>)>) -> Polynomial<MFp<N>> {
    let mut poly_res: Polynomial<MFp<N>> = Polynomial::new(vec![MFp::ZERO]);

    for (x_i, y_i) in points.iter() {
        let mut poly_nume_all: Polynomial<MFp<N>> = Polynomial::new(vec![MFp::ONE]);
        let mut poly_deno_all = MFp::ONE;
        for (x_j, _) in points.iter() {
            if x_i != x_j {
                let poly_nume: Polynomial<MFp<N>> =
                    Polynomial::new(vec![MFp::ONE, MFp::from(*x_j).neg()]);
                let poly_deno = MFp::from(*x_i) - MFp::from(*x_j);

                poly_nume_all *= poly_nume;
                poly_deno_all *= poly_deno;
            }
        }
        poly_res += Polynomial::<MFp<N>>::new(vec![*y_i])
            * (poly_nume_all * poly_deno_all.inverse().unwrap());
    }

    poly_res
}

pub fn generate_set<const N: u64>(ms_gen: u64, len: u64) -> Vec<MFp<N>> {
    (0..len).map(|i| exp_mod(ms_gen, i)).collect()
}

pub fn commit<const N: u64>(o: Vec<Polynomial<MFp<N>>>, d: u64, g: u64) -> Vec<MFp<N>> {
    let mut res = vec![];

    for poly in o {
        let mut res_poly = MFp::<N>::ONE;

        if let Degree::Num(deg) = poly.degree() {
            let mut s = MFp::<N>::from(g);
            let d = d % (N - 1);

            for i in 0..=deg {
                let coef = poly.terms[deg - i].into_bigint().0[0];
                let value = exp_mod::<N>(s.into_bigint().0[0], coef);
                res_poly *= value;
                s = exp_mod::<N>(s.into_bigint().0[0], d);
            }
        }

        if res_poly == MFp::ONE {
            res.push(MFp::<N>::from(g));
        } else {
            res.push(res_poly);
        }
    }

    res
}

pub fn get_poinsts_row<const N: u64>(
    mat: &DMatrix<MFp<N>>,
    h: &Vec<MFp<N>>,
    k: &Vec<MFp<N>>,
) -> Vec<(MFp<N>, MFp<N>)> {
    let mut points: Vec<(MFp<N>, MFp<N>)> = vec![];
    let mut c = 0;

    for i in 0..mat.nrows() {
        for j in 0..mat.ncols() {
            if mat[(i, j)] != MFp::ZERO {
                points.push((k[c], h[i]));
                c += 1;
            }
        }
    }

    points
}

pub fn get_poinsts_col<const N: u64>(
    mat: &DMatrix<MFp<N>>,
    h: &Vec<MFp<N>>,
    k: &Vec<MFp<N>>,
) -> Vec<(MFp<N>, MFp<N>)> {
    let mut points: Vec<(MFp<N>, MFp<N>)> = vec![];
    let mut c = 0;

    for i in 0..mat.nrows() {
        for j in 0..mat.ncols() {
            if mat[(i, j)] != MFp::ZERO {
                points.push((k[c], h[j]));
                c += 1;
            }
        }
    }

    points
}

pub fn get_poinsts_val<const N: u64>(
    mat: &DMatrix<MFp<N>>,
    k: &Vec<MFp<N>>,
) -> Vec<(MFp<N>, MFp<N>)> {
    let mut points: Vec<(MFp<N>, MFp<N>)> = vec![];
    let mut c = 0;

    for i in 0..mat.nrows() {
        for j in 0..mat.ncols() {
            if mat[(i, j)] != MFp::ZERO {
                points.push((k[c], mat[(i, j)]));
                c += 1;
            }
        }
    }

    points
}

pub fn generate_set_eval<const N: u64>(ms_gen: u64, n: usize, t: usize, len: usize) -> Vec<MFp<N>> {
    let mut set: Vec<MFp<N>> = vec![];

    for i in t..n {
        set.push(exp_mod(ms_gen, i as u64));
    }

    let zeros = len as isize - n as isize + t as isize;

    assert!(zeros >= 0);

    if zeros > 0 {
        for _ in 0..zeros {
            set.push(MFp::ZERO);
        }
    }

    set
}

pub fn get_points_set<const N: u64>(seq_k: &Vec<MFp<N>>, k: &Vec<MFp<N>>) -> Vec<(MFp<N>, MFp<N>)> {
    let mut points: Vec<(MFp<N>, MFp<N>)> = vec![];

    assert!(seq_k.len() == k.len(), "sets are not equal");

    for point in k.iter().zip(seq_k.iter()) {
        points.push((*point.0, *point.1));
    }

    points
}

pub fn parser(file_path: PathBuf) -> Result<Vec<Gate>> {
    let reader = open_file(&file_path)?;
    let gates = read_parse_lines(reader)?;
    Ok(gates)
}

fn open_file(file_path: &PathBuf) -> Result<BufReader<File>> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file))
}

fn read_parse_lines(reader: BufReader<File>) -> Result<Vec<Gate>> {
    let mut gates = Vec::new();

    for (index, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let (operation, operands) = parse_line(&line, index)?;
        // let gate_type = gate_type(operation)?;
        let gate_type = gate_type(operation);
        if let Err(ref e) = gate_type {
            // return Err(e);
            eprintln!("{}", e);
            continue;
        }

        let constant = operands.get(2).unwrap().parse::<u64>()?;
        let gate = Gate::new(index + 1, 0, None, Some(constant), gate_type?);
        gates.push(gate);
    }

    Ok(gates)
}

fn gate_type(op: &str) -> Result<GateType> {
    match op {
        "mul" => Ok(GateType::Mul),
        "addi" => Ok(GateType::Add),
        _ => Err(anyhow!("operation is not support: {}", op)),
    }
}

fn parse_line(line: &str, index: usize) -> Result<(&str, Vec<&str>)> {
    let parts: Vec<&str> = line
        .trim()
        .split(&[',', ' '])
        .filter(|s| !s.trim().is_empty())
        .collect();
    if parts.len() >= 6 {
        Ok((parts[2], parts[3..].to_vec()))
    } else {
        Err(anyhow!("a problem occurred in line {}", index))
    }
}

pub fn mat_to_vec<const N: u64>(
    mat: &nalgebra::Matrix<
        MFp<N>,
        nalgebra::Dyn,
        na::Const<1>,
        nalgebra::VecStorage<MFp<N>, nalgebra::Dyn, na::Const<1>>,
    >,
) -> Vec<MFp<N>> {
    assert!(mat.ncols() < 2, "cannot convet to vec mat.ncols() < 2");

    let mut v: Vec<MFp<N>> = vec![];

    for i in 0..mat.nrows() {
        v.push(mat[(i, 0)]);
    }

    v
}

pub fn parse_from_lines(line_file: &PathBuf, opcodes_file: &PathBuf) -> Result<Vec<Gate>> {
    let mut gates = Vec::new();

    let line_file = open_file(line_file).unwrap();

    for line in line_file.lines() {
        let line_num = line.unwrap().trim().parse::<usize>().unwrap();
        let gates_file = open_file(opcodes_file).unwrap();
        let line = gates_file.lines().nth(line_num - 1).unwrap().unwrap();
        let (operation, operands) = parse_line(&line, line_num).unwrap();
        let gate_type = gate_type(operation);
        if let Err(ref e) = gate_type {
            // return Err(e);
            eprintln!("Error: {}", e);
            continue;
        }
        let constant = operands.get(2).unwrap().parse::<u64>().unwrap();
        let gate = Gate::new(line_num, 0, None, Some(constant), gate_type.unwrap());
        gates.push(gate);
    }

    Ok(gates)
}

pub fn push_random_points<const N: u64>(
    points: &mut Vec<(MFp<N>, MFp<N>)>,
    b: u64,
    set_h: &HashSet<MFp<N>>,
) {
    let mut rng = thread_rng();
    for _ in 0..b {
        let d = gen_rand_not_in_set(set_h);
        let r = MFp::<N>::from(rng.gen_range(0..N));
        points.push((d, r));
    }
}

pub fn vec_to_hashset<const N: u64>(vec: &Vec<MFp<N>>) -> HashSet<MFp<N>> {
    vec.iter().cloned().collect()
}

pub fn gen_rand_not_in_set<const N: u64>(set: &HashSet<MFp<N>>) -> MFp<N> {
    let mut rng = rand::thread_rng();
    let mut num;

    loop {
        num = MFp::<N>::from(rng.gen_range(0..N));
        if !set.contains(&num) {
            break;
        }
    }

    num
}

pub fn vanishing_poly<const N: u64>(set: &Vec<MFp<N>>) -> Polynomial<MFp<N>> {
    let mut vp = Polynomial::new(vec![MFp::ONE]);

    for i in set {
        vp *= Polynomial::new(vec![MFp::ONE, MFp::ZERO]) - Polynomial::new(vec![*i]);
    }

    vp
}

pub fn poly_gen_randomly<const N: u64>(deg: usize) -> Polynomial<MFp<N>> {
    let mut rng = rand::thread_rng();
    let mut poly = vec![];

    for _ in 0..deg {
        poly.push(MFp::<N>::from(rng.gen_range(0..N)));
    }

    Polynomial::new(poly)
}

#[cfg(test)]
mod parser_test {
    use crate::parse_line;

    #[test]
    fn parse_line_func() {
        let line1 = "40380552:       02f407b3                mul     a1,s0,5";
        let line2 = "40380552:       02f407b3                mul     a1, s0, 5";
        let line3 = "40380552:       02f407b3                mul     a1  ,  s0  ,  5  ";

        let parse1 = parse_line(line1, 1).unwrap();
        let parse2 = parse_line(line2, 2).unwrap();
        let parse3 = parse_line(line3, 3).unwrap();

        assert_eq!(parse1, ("mul", ["a1", "s0", "5"].to_vec()));
        assert_eq!(parse2, ("mul", ["a1", "s0", "5"].to_vec()));
        assert_eq!(parse3, ("mul", ["a1", "s0", "5"].to_vec()));
    }
}
