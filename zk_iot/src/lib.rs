// incomplete ZKP scheme with "nalgebra" lib
// initializes and partially fills matrices A, B, C
extern crate nalgebra as na;
use ark_ff::{
    fields::{MontBackend, MontConfig},
    FpConfig,
};
use ark_ff::{BigInteger256, Field, Fp256, MontFp, PrimeField};
use ark_ff::{Fp, Fp64};
use na::{DMatrix, DVector};
use rustnomial::{Degree, Polynomial, SizedPolynomial};
use std::{fs::File, io::{self, BufRead, BufReader}, ops::Neg, path::PathBuf};
use std::u64;
use anyhow::{anyhow, Result};

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
    for i in 0..t as usize {
        for j in 0..mat.ncols() {
            mat[(i, j)] = MFp::ZERO;
        }
    }
}

fn ord(n: u64) -> u64 {
    n - 1
}

pub fn init<const N: u64>(gates: Vec<Gate>, ni: usize, a_mat: &mut DMatrix<MFp<N>>, b_mat: &mut DMatrix<MFp<N>>, c_mat: &mut DMatrix<MFp<N>>, z_poly: &mut DVector::<MFp<N>>) {
    for (i, gate) in gates.iter().enumerate() {
        let index = 1 + ni + i;
        c_mat[(index, index)] = MFp::ONE;

        match gate.gate_type {
            GateType::Add => {
                a_mat[(index, 0)] = MFp::ONE;

                b_mat[(index, gate.inx_left)] = if let Some(val) = gate.val_left {
                    MFp::from(val)
                } else {
                    MFp::ONE
                };
                b_mat[(index, gate.inx_right)] = if let Some(val) = gate.val_right {
                    MFp::from(val)
                } else {
                    MFp::ONE
                };

                z_poly[i + 2] = z_poly[i + 1]
                    + if let Some(val) = gate.val_right {
                        MFp::from(val)
                    } else {
                        MFp::ZERO
                    };
            }
            GateType::Mul => {
                a_mat[(index, gate.inx_left)] = if let Some(val) = gate.val_left {
                    MFp::from(val)
                } else {
                    MFp::ONE
                };
                b_mat[(index, gate.inx_right)] = if let Some(val) = gate.val_right {
                    MFp::from(val)
                } else {
                    MFp::ONE
                };

                z_poly[i + 2] = z_poly[i + 1]
                    * if let Some(val) = gate.val_right {
                        MFp::from(val)
                    } else {
                        MFp::ONE
                    };
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

// this function calculates the exponent of a given number modulo (N - 1)
pub fn exp_calc<const N: u64>(exp: u64) -> u64 {
    let mut res = 0;
    let mut c = exp;
    while c % (N - 1) != 0 {
        res += 1;
        c -= 1;
    }
    res
}

pub fn print_mat<const N: u64>(mat: &DMatrix<MFp<N>>) {
    for i in 0..mat.nrows() {
        for j in 0..mat.ncols() {
            let derr = mat[(i, j)];
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
    let mut set: Vec<MFp<N>> = vec![];
    for i in 0..len {
        set.push(exp_mod(ms_gen, i));
    }
    set
}


pub fn commit<const N: u64>(o: Vec<Polynomial<MFp<N>>>, d: u64, g: u64) -> Vec<MFp<N>> {
    let mut res = vec![];

    for poly in o {
        let mut res_poly = MFp::<N>::ONE;

        if let Degree::Num(deg) = poly.degree() {
            for i in 0..=deg as u32 {
                let exp = d.pow(i);
                let exp = exp_calc::<N>(exp);
                let value = exp_mod::<N>(g, exp).into_bigint().0[0];
                let coef = poly.terms[deg - i as usize].into_bigint().0[0];

                let value = exp_mod::<N>(value, coef);

                res_poly *= value;
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

pub 

fn get_poinsts_row<const N: u64>(mat: &DMatrix<MFp<N>>, h: &Vec<MFp<N>> , k: &Vec<MFp<N>>) -> Vec<(MFp<N>, MFp<N>)> {
    let mut points: Vec<(MFp<N>, MFp<N>)> = vec![];
    let mut c = 0;

    for i in 0..mat.nrows() {
        for j in 0..mat.ncols() {
            if mat[(i, j)] != MFp::ZERO {
                points.push((k[c],  h[i]));
                c += 1;
            }
        }
    }

    points
}


pub fn get_poinsts_col<const N: u64>(mat: &DMatrix<MFp<N>>, h: &Vec<MFp<N>> , k: &Vec<MFp<N>>) -> Vec<(MFp<N>, MFp<N>)> {
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


pub fn get_poinsts_val<const N: u64>(mat: &DMatrix<MFp<N>>, h: &Vec<MFp<N>> , k: &Vec<MFp<N>>) -> Vec<(MFp<N>, MFp<N>)> {
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

    // assert!(zeros >= 0);
    
    if zeros > 0 {
        for _ in 0..zeros {
            set.push(MFp::ZERO);
        }
    }

    set
}


pub fn get_points_set<const N: u64>(seq_k: &Vec<MFp<N>> , k: &Vec<MFp<N>>) -> Vec<(MFp<N>, MFp<N>)> {
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
        if let Some((operation, operands)) = parse_line(&line, index)? {
            let gate_type = match operation {
                "mul" => GateType::Mul,
                "addi" => GateType::Add,
                _ => continue,
            };

            let constant = operands.get(2).unwrap().parse::<u64>()?;
            let gate = Gate::new(index + 1, 0, None, Some(constant), gate_type);
            gates.push(gate);
        }
    }

    Ok(gates)
}

fn parse_line(line: &str, index: usize) -> Result<Option<(&str, Vec<&str>)>> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 4 {
        let operation = *parts.get(2).ok_or_else(|| {
            anyhow!("Operation not found in line {}", index + 1)
        })?;
        let operands: Vec<&str> = parts.get(3).unwrap().split(',').collect();
        Ok(Some((operation, operands)))
    } else {
        Ok(None)
    }
}