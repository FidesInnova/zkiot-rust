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


use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Read},
    path::PathBuf,
};

use crate::{
    dsp_mat, dsp_poly, dsp_vec, json_file::{open_file, write_term, ClassData}, math::*, parser::{Gate, GateType, RegData}, print_dbg, println_dbg, to_bint, utils::*
};
use anyhow::Result;
use ark_ff::Field;
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Commitment {
    pub set_h: Vec<Mfp>,
    pub set_k: Vec<Mfp>,
    pub numebr_t_zero: usize,
    pub matrices: Matrices,
    pub polys_px: Vec<Poly>,
    pub points_px: Vec<HashMap<Mfp, Mfp>>,
}

impl Commitment {
    // Constructor method Generate sets and Initilize matrices
    pub fn new(class_data: ClassData) -> CommitmentBuilder {
        let set_h_len: u64 = (class_data.n_g + class_data.n_i + 1).try_into().unwrap();
        let numebr_t_zero: u64 = (class_data.n_i + 1).try_into().unwrap(); // Number of rows (|x| = self.numebr_t_zero, where self.numebr_t_zero = ni + 1)
        let set_k_len = ((set_h_len * set_h_len - set_h_len) / 2)
            - ((numebr_t_zero * numebr_t_zero - numebr_t_zero) / 2);

        let set_h = generate_set(set_h_len);
        let set_k = generate_set(set_k_len);

        let matrix_size = class_data.n_g + class_data.n_i + 1;
        let matrices = Matrices::new(matrix_size.try_into().unwrap());

        CommitmentBuilder {
            commitm: Commitment {
                set_h,
                set_k,
                numebr_t_zero: numebr_t_zero.try_into().unwrap(),
                matrices,
                polys_px: vec![],
                points_px: vec![],
            },
        }
    }

    /// Generates a commitment based on the AHP commitment generation process.
    /// For more details, see:
    /// [AHP Commitment Generation Documentation](https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/2-commitment-phase#id-2-3-ahp-commitment)
    pub fn get_polynomials_commitment(
        &self,
        generator: u64,
        commitment_key: &Vec<Mfp>,
    ) -> Vec<Mfp> {
        let commitment = compute_all_commitment(&self.polys_px, commitment_key, generator);
        println_dbg!("com_ahp: {}", dsp_vec!(commitment));
        commitment
    }

    pub fn get_matrix_az(&self) -> DMatrix<Mfp> {
        &self.matrices.a * &self.matrices.z
    }

    pub fn get_matrix_bz(&self) -> DMatrix<Mfp> {
        &self.matrices.b * &self.matrices.z
    }

    pub fn get_matrix_cz(&self) -> DMatrix<Mfp> {
        &self.matrices.c * &self.matrices.z
    }

    /// Retrieves a vector containing the results of multiplying matrices `a`, `b`, and `c` with matrix `z`.
    /// 
    /// The returned array contains three vectors:
    /// - Index 0: Result of `a * z`
    /// - Index 1: Result of `b * z`
    /// - Index 2: Result of `c * z`
    /// 
    /// # Returns
    /// An array of vectors, where each vector contains the results of the respective matrix multiplication.
    pub fn get_matrix_oz_vec(&self) -> [Vec<Mfp>; 3] {
        [
            mat_to_vec(&self.get_matrix_az()),
            mat_to_vec(&self.get_matrix_bz()),
            mat_to_vec(&self.get_matrix_cz()),
        ]
    }

    /// Store in Json file
    pub fn store(&self, path: &str) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        let commitment_json = CommitmentJson::new(&self.points_px, &self.polys_px);
        serde_json::to_writer(writer, &commitment_json)?;
        Ok(())
    }

    /// Restore Commitment from Json file
    pub fn restore(path: &str) -> Result<CommitmentJson> {
        let reader = open_file(&PathBuf::from(path))?;
        let commitment_json: CommitmentJson = serde_json::from_reader(reader)?;
        Ok(commitment_json)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// A struct representing a commitment in JSON format, containing points and polynomial data.
pub struct CommitmentJson {
    points_px: Vec<Vec<(u64, u64)>>,
    polys_px: Vec<Vec<u64>>,
}
impl CommitmentJson {
    pub fn new(points_px: &Vec<HashMap<Mfp, Mfp>>, polys_px: &Vec<Poly>) -> Self {
        // Extract values for CommitmentJson from the Commitment struct
        let polys_px_t: Vec<Vec<u64>> = polys_px.iter().map(|p| write_term(p)).collect();
        let points_px_t: Vec<Vec<(u64, u64)>> = points_px
            .iter()
            .map(|points| {
                points
                    .iter()
                    .map(|(&key, &val)| (to_bint!(key), to_bint!(val)))
                    .collect()
            })
            .collect();

        Self {
            points_px: points_px_t,
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

    /// Retrieves the points data as a vector of hash maps.
    /// 
    /// # Returns
    /// A vector of hash maps where each map represents a set of points with `Mfp` keys and values.
    pub fn get_points_px(&self) -> Vec<HashMap<Mfp, Mfp>> {
        self.points_px
            .iter()
            .map(|points| {
                points
                    .iter()
                    .map(|&p| (Mfp::from(p.0), Mfp::from(p.1)))
                    .collect()
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
/// A struct representing a collection of matrices used in computations.
pub struct Matrices {
    pub a: DMatrix<Mfp>,
    pub b: DMatrix<Mfp>,
    pub c: DMatrix<Mfp>,
    pub z: DMatrix<Mfp>,
    pub size: usize,
}

impl Matrices {
    pub fn new(size: usize) -> Self {
        let a = DMatrix::<Mfp>::zeros(size, size);
        let b = DMatrix::<Mfp>::zeros(size, size);
        let c = DMatrix::<Mfp>::zeros(size, size);
        let z = DMatrix::<Mfp>::zeros(size, 1);

        Self { a, b, c, z, size }
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
    /// Initializes matrices A, B, C, Z  based on gate definitions.
    ///
    /// # Parameters
    /// - `gates`: A vector of `Gate` structs containing gate definitions.
    /// - `ni`: Number of inputs (registers).
    /// - `a_mat`: Mutable reference to matrix A to be updated.
    /// - `b_mat`: Mutable reference to matrix B to be updated.
    /// - `c_mat`: Mutable reference to matrix C to be updated.
    /// - `z_mat`: Mutable reference to matrix Z to be updated.
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
        // Initialize matrices A, B, C and z based on parsed gates
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
            if gate.gate_type == GateType::Ld {
                let right_val = gate.val_right.map_or(Mfp::ZERO, Mfp::from);
                match regs_data.contains_key(&gate.reg_right) {
                    true => panic!("The register has been loaded again!"),
                    false => regs_data.insert(gate.reg_right, RegData::new(right_val)),
                };
                println_dbg!("Ld: {}", right_val);
                // TODO: Determine if left_val is necessary for ld operation.
                continue;
            }
            println_dbg!("Gate Loop: {} ------------", counter);
            println_dbg!("Register: l:{}, r:{}", gate.reg_left, gate.reg_right);

            // Set index
            _index = 1 + ni + counter;


            let (mut li, mut ri) = (false, false);
            if !regs_data.get(&gate.reg_left).unwrap().witness.is_empty() {
                li = true;
            }
            if !regs_data.get(&gate.reg_right).unwrap().witness.is_empty() {
                ri = true;
            }

            // if gate.val_left.is_some() {
            //     li = true;
            // } 
            // if gate.val_right.is_some() {
            //     ri = true;
            // }

            // inx_left = if li { 
            //     let inx = (0..=32).fold(0,  |acc, x| acc + regs_data.get(&x).unwrap_or(&RegData::new(Mfp::ZERO)).witness.len()) + ni;
            //     println_dbg!("left:   index = {:<5}", inx);
            //     inx
            // } else {
            //     println_dbg!("left: None, index = {}", inx_left + 1);
            //     inx_left + 1
            // };

            // inx_right = if ri {
            //     let inx = (0..=32).fold(0, |acc, x| acc + regs_data.get(&x).unwrap_or(&RegData::new(Mfp::ZERO)).witness.len()) + ni;
            //     println_dbg!("right:  index = {:<5}", inx);
            //     inx
            // } else {
            //     println_dbg!("right: None, index = {}", inx_right + 1);
            //     inx_right + 1
            // };


            // It works better
            inx_left = if li {
                let inx = (0..=gate.reg_left).fold(0,  |acc, x| acc + regs_data.get(&x).unwrap_or(&RegData::new(Mfp::ZERO)).witness.len()) + ni;
                println_dbg!("left:   index = {:<5}", inx);
                inx
            } else {
                println_dbg!("left: None, index = {}", inx_left + 1);
                inx_left + 1
            };

            inx_right = if ri {
                let inx = (0..=gate.reg_right).fold(0, |acc, x| acc + regs_data.get(&x).unwrap_or(&RegData::new(Mfp::ZERO)).witness.len()) + ni;
                println_dbg!("right:  index = {:<5}", inx);
                inx
            } else {
                println_dbg!("right: None, index = {}", inx_right + 1);
                inx_right + 1
            };

            Self::add_val(&mut regs_data, gate, gate.gate_type, &mut val_counter);
            

            // inx_left += 1;
            // inx_right += 1;


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
                    println_dbg!("Gate: Div");
                    println_dbg!("Left:  A[{}, {}] = {}", _index, inx_left, invers_val(left_val));
                    a_mat[(_index, inx_left)] = invers_val(left_val);

                    println_dbg!("Right: B[{}, {}] = {}", _index, inx_right, invers_val(right_val));
                    b_mat[(_index, inx_right)] = invers_val(right_val);
                }
                _ => panic!("Invalid gate {:?}", gate.gate_type)
            }
            counter += 1;
        }

        // Set specific rows in matrices A, B, C to zero
        rows_to_zero(&mut self.commitm.matrices.a, self.commitm.numebr_t_zero);
        rows_to_zero(&mut self.commitm.matrices.b, self.commitm.numebr_t_zero);
        rows_to_zero(&mut self.commitm.matrices.c, self.commitm.numebr_t_zero);


        Self::gen_z_mat(&mut self.commitm.matrices.z, &regs_data);

        println_dbg!("Mat A:");
        dsp_mat!(self.commitm.matrices.a);
        println_dbg!("Mat B:");
        dsp_mat!(self.commitm.matrices.b);
        println_dbg!("Mat C:");
        dsp_mat!(self.commitm.matrices.c);
        println_dbg!("Mat Z:");
        dsp_mat!(self.commitm.matrices.z);

        self.clone()
    }

    fn add_val(regs_data: &mut HashMap<u8, RegData>, gate: &Gate, operator: GateType, val_counter: &mut usize) {
        if let Some(left_val) = gate.val_left {
            if let Some(reg) = regs_data.get_mut(&gate.reg_right) {
                let new_value = match reg.witness.last() {
                    // FIXME: Correct left and right position
                    Some(&(_,val)) => Self::apply_operator(val, Mfp::from(left_val), operator),
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
                    Some(&(_,val)) => Self::apply_operator(val, Mfp::from(right_val), operator),
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

    fn gen_z_mat(z_vec: &mut DMatrix<Mfp>, regs_data: &HashMap<u8, RegData>) {
        z_vec[(0, 0)] = Mfp::ONE;
        let mut z_vec_counter: usize = 1;

        let mut witnesses: Vec<(usize, Mfp)> = vec![];
        let mut final_val = vec![];
        for reg in 0..32 {
            if regs_data.contains_key(&reg) {
                let data = regs_data.get(&reg).unwrap();
                // println_dbg!("data here ==> {:?}", data);
                z_vec[(z_vec_counter, 0)] = data.init_val;
                z_vec_counter += 1;
                let mut witness = data.witness.clone();
                if witness.is_empty() {
                    continue;
                }
                let last_val = witness.pop().unwrap();
                witnesses.extend(witness.iter());
                final_val.push(last_val.1);
            }
        }

        witnesses.sort();

        for w in witnesses {
            z_vec[(z_vec_counter, 0)] = w.1;
            // println!("w: {}", w);
            z_vec_counter += 1;
        }
        


        for f in final_val.iter().rev() {
            z_vec[(z_vec_counter, 0)] = *f;
            z_vec_counter += 1;
        }
    }

    pub fn gen_polynomials(&mut self) -> Self {
        // TODO: Random values were taken from WIKI. After the test is completed, these inserts should be deleted or commented out.
        // Wiki link: (https://fidesinnova-1.gitbook.io/fidesinnova-docs/zero-knowledge-proof-zkp-scheme/2-commitment-phase#id-2-5-2-ahp-commitment)
        // Matrix A:
        let mut points_row_p_a = get_matrix_point_row(
            &self.commitm.matrices.a,
            &self.commitm.set_h,
            &self.commitm.set_k,
        );
        // rowA' = (48, 1), (73, 135), (62, 125), (132, 59), (65, 42), (80, 1)
        // points_row_p_a.insert(Mfp::from(48), Mfp::from(1));
        // points_row_p_a.insert(Mfp::from(73), Mfp::from(135));
        // points_row_p_a.insert(Mfp::from(62), Mfp::from(125));
        // points_row_p_a.insert(Mfp::from(132), Mfp::from(59));
        // points_row_p_a.insert(Mfp::from(65), Mfp::from(42));
        // points_row_p_a.insert(Mfp::from(80), Mfp::from(1));

        let mut points_col_p_a = get_matrix_point_col(
            &self.commitm.matrices.a,
            &self.commitm.set_h,
            &self.commitm.set_k,
        );
        // colA' = (48, 42), (73, 1), (62, 135), (132, 125), (65, 59), (80, 42)
        // points_col_p_a.insert(Mfp::from(48), Mfp::from(42));
        // points_col_p_a.insert(Mfp::from(73), Mfp::from(1));
        // points_col_p_a.insert(Mfp::from(62), Mfp::from(135));
        // points_col_p_a.insert(Mfp::from(132), Mfp::from(125));
        // points_col_p_a.insert(Mfp::from(65), Mfp::from(59));
        // points_col_p_a.insert(Mfp::from(80), Mfp::from(42));

        let points_val_p_a = get_matrix_point_val(
            &self.commitm.matrices.a,
            &self.commitm.set_h,
            &self.commitm.set_k,
            &points_row_p_a,
            &points_col_p_a,
        );

        // Matrix B:
        let mut points_row_p_b = get_matrix_point_row(
            &self.commitm.matrices.b,
            &self.commitm.set_h,
            &self.commitm.set_k,
        );
        // rowB' = (73, 59), (62, 1), (132, 42), (65, 135), (80, 59)
        // points_row_p_b.insert(Mfp::from(73), Mfp::from(59));
        // points_row_p_b.insert(Mfp::from(62), Mfp::from(1));
        // points_row_p_b.insert(Mfp::from(132), Mfp::from(42));
        // points_row_p_b.insert(Mfp::from(65), Mfp::from(135));
        // points_row_p_b.insert(Mfp::from(80), Mfp::from(59));

        let mut points_col_p_b = get_matrix_point_col(
            &self.commitm.matrices.b,
            &self.commitm.set_h,
            &self.commitm.set_k,
        );
        // colB' = (73, 59), (62, 42), (132, 125), (65, 1), (80, 135)
        // points_col_p_b.insert(Mfp::from(73), Mfp::from(59));
        // points_col_p_b.insert(Mfp::from(62), Mfp::from(42));
        // points_col_p_b.insert(Mfp::from(132), Mfp::from(125));
        // points_col_p_b.insert(Mfp::from(65), Mfp::from(1));
        // points_col_p_b.insert(Mfp::from(80), Mfp::from(135));

        let points_val_p_b = get_matrix_point_val(
            &self.commitm.matrices.b,
            &self.commitm.set_h,
            &self.commitm.set_k,
            &points_row_p_b,
            &points_col_p_b,
        );

        // Matrix C
        let mut points_row_p_c = get_matrix_point_row(
            &self.commitm.matrices.c,
            &self.commitm.set_h,
            &self.commitm.set_k,
        );
        // FIXME: Wiki
        // rowC' = (48, 1), (73, 59), (62, 125), (132, 1), (65, 135), (80, 42)
        // points_row_p_c.insert(Mfp::from(48), Mfp::from(1));
        // points_row_p_c.insert(Mfp::from(73), Mfp::from(59));
        // points_row_p_c.insert(Mfp::from(62), Mfp::from(125));
        // points_row_p_c.insert(Mfp::from(132), Mfp::from(1));
        // points_row_p_c.insert(Mfp::from(65), Mfp::from(135));
        // points_row_p_c.insert(Mfp::from(80), Mfp::from(42));

        let mut points_col_p_c = get_matrix_point_col(
            &self.commitm.matrices.c,
            &self.commitm.set_h,
            &self.commitm.set_k,
        );
        // FIXME: Wiki
        // colC' = (48, 125), (73, 59), (62, 1), (132, 1), (65, 42), (80, 59)
        // points_col_p_c.insert(Mfp::from(48), Mfp::from(125));
        // points_col_p_c.insert(Mfp::from(73), Mfp::from(59));
        // points_col_p_c.insert(Mfp::from(62), Mfp::from(1));
        // points_col_p_c.insert(Mfp::from(132), Mfp::from(1));
        // points_col_p_c.insert(Mfp::from(65), Mfp::from(42));
        // points_col_p_c.insert(Mfp::from(80), Mfp::from(59));

        let points_val_p_c = get_matrix_point_val(
            &self.commitm.matrices.c,
            &self.commitm.set_h,
            &self.commitm.set_k,
            &points_row_p_c,
            &points_col_p_c,
        );

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

        let points_vector = vec![
            points_val_p_a,
            points_row_p_a,
            points_col_p_a,
            points_val_p_b,
            points_row_p_b,
            points_col_p_b,
            points_val_p_c,
            points_row_p_c,
            points_col_p_c,
        ];

        self.commitm.points_px = points_vector;
        self.commitm.polys_px = polys_pxs;

        self.clone()
    }

    pub fn build(&self) -> Commitment {
        Commitment {
            ..self.commitm.clone()
        }
    }
}
