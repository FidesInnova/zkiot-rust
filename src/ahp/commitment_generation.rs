// Copyright 2024 Fidesinnova, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::Result;
use ark_ff::Field;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use crate::dsp_mat;
use crate::dsp_poly;
use crate::dsp_vec;
use crate::json_file::open_file;
use crate::json_file::write_term;
use crate::json_file::ClassData;
use crate::math::*;
use crate::matrices::matrix_size;
use crate::matrices::matrix_t_zeros;
use crate::matrices::Matrices;
use crate::parser::Gate;
use crate::parser::GateType;
use crate::parser::RegData;
use crate::print_dbg;
use crate::println_dbg;
use crate::to_bint;
use crate::utils::*;

#[derive(Debug, Clone)]
pub struct Commitment {
    pub set_h: Vec<Mfp>,
    pub set_k: Vec<Mfp>,
    pub numebr_t_zero: usize,
    pub matrices: Matrices,
    pub polys_px: Vec<Poly>,
}

impl Commitment {
    /// Constructor method Generate sets and Initilize matrices
    pub fn new(class_data: ClassData) -> CommitmentBuilder {
        let numebr_t_zero = matrix_t_zeros(&class_data) as u64;

        let set_h = generate_set(class_data.n);
        let set_k = generate_set(class_data.m);

        let matrix_size = matrix_size(&class_data);
        let matrices = Matrices::new(matrix_size.try_into().unwrap());

        CommitmentBuilder {
            commitm: Commitment {
                set_h,
                set_k,
                numebr_t_zero: numebr_t_zero.try_into().unwrap(),
                matrices,
                polys_px: vec![],
            },
        }
    }

    /// Generates a commitment based on the AHP commitment generation process.
    /// For more details, see:
    /// [AHP Commitment Generation Documentation](https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/2-commitment-phase#id-2-3-ahp-commitment)
    pub fn get_polynomials_commitment(&self, commitment_key: &Vec<Mfp>) -> Vec<Mfp> {
        let commitment = compute_all_commitment(&self.polys_px, commitment_key);
        println_dbg!("com_ahp: {}", dsp_vec!(commitment));
        commitment
    }

    /// Store in Json file
    pub fn store(&self, path: &str) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        let commitment_json = CommitmentJson::new(&self.polys_px);
        serde_json::to_writer(writer, &commitment_json)?;
        Ok(())
    }

    /// Restore Commitment from Json file
    pub fn restore(path: &str) -> Result<CommitmentJson> {
        read_json_file(path)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// A struct representing a commitment in JSON format, containing points and polynomial data.
pub struct CommitmentJson {
    polys_px: Vec<Vec<u64>>,
}
impl CommitmentJson {
    pub fn new(polys_px: &Vec<Poly>) -> Self {
        // Extract values for CommitmentJson from the Commitment struct
        let polys_px_t: Vec<Vec<u64>> = polys_px.iter().map(|p| write_term(p)).collect();

        Self {
            polys_px: polys_px_t,
        }
    }

    /// Retrieves the polynomial data as a vector of `Poly` instances.
    ///
    /// # Returns
    /// A vector of `Poly` objects constructed from the polynomial coefficients stored in `polys_px`.
    pub fn get_polys_px(&self) -> Vec<Poly> {
        self.polys_px
            .iter()
            .map(|v| {
                let mut poly =
                    Poly::from(v.iter().rev().map(|&t| Mfp::from(t)).collect::<Vec<Mfp>>());
                poly.trim();
                poly
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
/// A struct for building a `Commitment`.
///
/// This struct encapsulates a `Commitment` instance, providing methods to construct
/// and manipulate commitments in a structured manner.
pub struct CommitmentBuilder {
    commitm: Commitment,
}

impl CommitmentBuilder {
    /// Initializes matrices A, B, C  based on gate definitions.
    ///
    /// # Parameters
    /// - `gates`: A vector of `Gate` structs containing gate definitions.
    /// - `ni`: Number of inputs (registers).
    /// - `a_mat`: Mutable reference to matrix A to be updated.
    /// - `b_mat`: Mutable reference to matrix B to be updated.
    /// - `c_mat`: Mutable reference to matrix C to be updated.
    ///
    /// # Description
    /// This function iterates through the provided `gates` vector and updates the matrices
    /// A, B, and C as well as the polynomial matrix `z_mat` based on the type of each gate:
    /// - **Add** gates: Updates matrices and modifies `z_mat` with addition.
    /// - **Mul** gates: Updates matrices and modifies `z_mat` with multiplication.
    ///
    /// The matrices are populated with values according to the gate definitions, and the
    /// `z_mat` matrix is updated with the results of operations specified by the gates.
    ///
    /// For further details, please refer to the documentation:
    /// [Documentation Link](https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/2-commitment-phase)
    pub fn gen_matrices(&mut self, gates: Vec<Gate>, number_inputs: usize) -> Self {
        // Initialize matrices A, B, C based on parsed gates
        let ni = number_inputs;
        let a_mat = &mut self.commitm.matrices.a;
        let b_mat = &mut self.commitm.matrices.b;
        let c_mat = &mut self.commitm.matrices.c;

        let mut regs_data: HashMap<u8, RegData> = HashMap::new();

        let mut _index = 0;
        let mut counter = 0;
        let mut inx_left = 0;
        let mut inx_right = 0;
        let mut val_counter: usize = 0;

        for (_, gate) in gates.iter().enumerate() {
            if !regs_data.contains_key(&gate.reg_right) {
                regs_data.insert(gate.reg_right, RegData::new(Mfp::ONE));
            }

            println_dbg!("Gate Loop: {} ------------", counter);
            println_dbg!("Register: {}", gate.reg_left);

            // Set index
            _index = 1 + ni + counter;

            inx_left = if !regs_data.get(&gate.reg_left).unwrap().witness.is_empty() {
                let inx = (0..=gate.reg_left).fold(0, |acc, x| {
                    acc + regs_data
                        .get(&x)
                        .unwrap_or(&RegData::new(Mfp::ZERO))
                        .witness
                        .len()
                }) + ni;
                inx
            } else {
                inx_left + 1
            };

            inx_right = if !regs_data.get(&gate.reg_right).unwrap().witness.is_empty() {
                let inx = (0..=gate.reg_right).fold(0, |acc, x| {
                    acc + regs_data
                        .get(&x)
                        .unwrap_or(&RegData::new(Mfp::ZERO))
                        .witness
                        .len()
                }) + ni;
                inx
            } else {
                inx_right + 1
            };

            Self::add_val(&mut regs_data, gate, gate.gate_type, &mut val_counter);

            let left_val = if let Some(val) = gate.val_left {
                inx_left = 0;
                println_dbg!("* left:  index = 0    , val = {}", val);
                Mfp::from(val)
            } else {
                println_dbg!("* left:  index = {:<5}, val = None = 1", inx_left);
                Mfp::ONE
            };
            let right_val = if let Some(val) = gate.val_right {
                inx_right = 0;
                println_dbg!("* right: index = 0    , val = {}", val);
                Mfp::from(val)
            } else {
                println_dbg!("* right: index = {:<5}, val = None = 1", inx_right);
                Mfp::ONE
            };

            c_mat[(_index, _index)] = Mfp::ONE;
            println_dbg!("C[{}, {}] = 1", _index, _index);

            match gate.gate_type {
                GateType::Add => {
                    println_dbg!("Gate: Add");
                    println_dbg!("A[{}, 0] = 1", _index);
                    a_mat[(_index, 0)] = Mfp::ONE;

                    println_dbg!("Left:  B[{}, {}] = {}", _index, inx_left, left_val);
                    b_mat[(_index, inx_left)] = left_val;

                    println_dbg!("Right: B[{}, {}] = {}", _index, inx_right, right_val);
                    b_mat[(_index, inx_right)] = right_val;
                }
                GateType::Mul => {
                    println_dbg!("Gate: Mul");
                    println_dbg!("Left:  A[{}, {}] = {}", _index, inx_left, left_val);
                    a_mat[(_index, inx_left)] = left_val;

                    println_dbg!("Right: B[{}, {}] = {}", _index, inx_right, right_val);
                    b_mat[(_index, inx_right)] = right_val;
                }
                GateType::Sub => {
                    // FIXME: This instruction has mathematical flaws and should not be used
                    println_dbg!("Gate: Sub");
                    println_dbg!("A[{}, 0] = 1", _index);
                    a_mat[(_index, 0)] = Mfp::ONE;

                    print_dbg!("Left:  B[{}, {}] = ", _index, inx_left);
                    b_mat[(_index, inx_left)] = match to_bint!(left_val) {
                        1 => Mfp::ONE,
                        _ => -left_val,
                    };
                    println_dbg!("{}", b_mat[(_index, inx_left)]);

                    print_dbg!("Right: B[{}, {}] = ", _index, inx_right);
                    b_mat[(_index, inx_right)] = match to_bint!(right_val) {
                        1 => Mfp::ONE,
                        _ => -right_val,
                    };
                    println_dbg!("{}", b_mat[(_index, inx_right)]);
                }
                GateType::Div => {
                    // FIXME: This instruction has mathematical flaws and should not be used
                    println_dbg!("Gate: Div");
                    println_dbg!(
                        "Left:  A[{}, {}] = {}",
                        _index,
                        inx_left,
                        invers_val(left_val)
                    );
                    a_mat[(_index, inx_left)] = invers_val(left_val);

                    println_dbg!(
                        "Right: B[{}, {}] = {}",
                        _index,
                        inx_right,
                        invers_val(right_val)
                    );
                    b_mat[(_index, inx_right)] = invers_val(right_val);
                }
                _ => panic!("Invalid gate {:?}", gate.gate_type),
            }
            counter += 1;
        }

        // Set specific rows in matrices A, B, C to zero
        rows_to_zero(&mut self.commitm.matrices.a, self.commitm.numebr_t_zero);
        rows_to_zero(&mut self.commitm.matrices.b, self.commitm.numebr_t_zero);
        rows_to_zero(&mut self.commitm.matrices.c, self.commitm.numebr_t_zero);

        // Print matrices if the program is compiled in debug mode
        println_dbg!("Mat A:");
        dsp_mat!(self.commitm.matrices.a);
        println_dbg!("Mat B:");
        dsp_mat!(self.commitm.matrices.b);
        println_dbg!("Mat C:");
        dsp_mat!(self.commitm.matrices.c);

        self.clone()
    }

    pub fn add_val(
        regs_data: &mut HashMap<u8, RegData>,
        gate: &Gate,
        operator: GateType,
        val_counter: &mut usize,
    ) {
        if let Some(left_val) = gate.val_left {
            if let Some(reg) = regs_data.get_mut(&gate.reg_left) {
                let new_value = match reg.witness.last() {
                    // FIXME: Correct left and right position
                    Some(&(_, val)) => Self::apply_operator(val, Mfp::from(left_val), operator),
                    None => Self::apply_operator(reg.init_val, Mfp::from(left_val), operator),
                };
                reg.witness.push((*val_counter, new_value));
                *val_counter += 1;
                println!("new_val: {}", new_value);
            }
        }
        if let Some(right_val) = gate.val_right {
            if let Some(reg) = regs_data.get_mut(&gate.reg_right) {
                let new_value = match reg.witness.last() {
                    Some(&(_, val)) => Self::apply_operator(val, Mfp::from(right_val), operator),
                    None => Self::apply_operator(reg.init_val, Mfp::from(right_val), operator),
                };
                reg.witness.push((*val_counter, new_value));
                *val_counter += 1;
                println!("new_val: {}", new_value);
            }
        }
    }

    fn apply_operator(l: Mfp, r: Mfp, operator: GateType) -> Mfp {
        match operator {
            GateType::Add => l + r,
            GateType::Sub => l - r,
            GateType::Mul => l * r,
            GateType::Div => div_mod_val(l, r),
            GateType::Ld => panic!("Invalid operation for Ld gate type"),
        }
    }

    pub fn gen_polynomials(&mut self) -> Self {
        let set_h = &self.commitm.set_h;
        let set_k = &self.commitm.set_k;

        // Collect row, column, and value points from matrix A
        let (points_row_p_a, points_col_p_a, points_val_p_a) = get_matrix_points(&self.commitm.matrices.a, set_h, set_k);
        // Collect row, column, and value points from matrix B
        let (points_row_p_b, points_col_p_b, points_val_p_b) = get_matrix_points(&self.commitm.matrices.b, set_h, set_k);
        // Collect row, column, and value points from matrix C.
        let (points_row_p_c, points_col_p_c, points_val_p_c) = get_matrix_points(&self.commitm.matrices.c, set_h, set_k);

        let a_row_px = sigma_yi_li(&points_row_p_a, &self.commitm.set_k);
        println_dbg!("a_row_px: ");
        dsp_poly!(a_row_px);
        let a_col_px = sigma_yi_li(&points_col_p_a, &self.commitm.set_k);
        println_dbg!("a_col_px: ");
        dsp_poly!(a_col_px);
        let a_val_px = sigma_yi_li(&points_val_p_a, &self.commitm.set_k);
        println_dbg!("a_val_px: ");
        dsp_poly!(a_val_px);

        let b_row_px = sigma_yi_li(&points_row_p_b, &self.commitm.set_k);
        println_dbg!("b_row_px: ");
        dsp_poly!(b_row_px);
        let b_col_px = sigma_yi_li(&points_col_p_b, &self.commitm.set_k);
        println_dbg!("b_col_px: ");
        dsp_poly!(b_col_px);
        let b_val_px = sigma_yi_li(&points_val_p_b, &self.commitm.set_k);
        println_dbg!("b_val_px: ");
        dsp_poly!(b_val_px);

        let c_row_px = sigma_yi_li(&points_row_p_c, &self.commitm.set_k);
        println_dbg!("c_row_px: ");
        dsp_poly!(c_row_px);
        let c_col_px = sigma_yi_li(&points_col_p_c, &self.commitm.set_k);
        println_dbg!("c_col_px: ");
        dsp_poly!(c_col_px);
        let c_val_px = sigma_yi_li(&points_val_p_c, &self.commitm.set_k);
        println_dbg!("c_val_px: ");
        dsp_poly!(c_val_px);

        let polys_pxs = vec![
            a_row_px, a_col_px, a_val_px, b_row_px, b_col_px, b_val_px, c_row_px, c_col_px,
            c_val_px,
        ];

        self.commitm.polys_px = polys_pxs;

        self.clone()
    }

    pub fn build(&self) -> Commitment {
        Commitment {
            ..self.commitm.clone()
        }
    }
}
