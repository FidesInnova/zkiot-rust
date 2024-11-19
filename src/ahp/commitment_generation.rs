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
use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use crate::dsp_mat;
use crate::dsp_poly;
use crate::dsp_vec;
use crate::json_file::open_file;
use crate::json_file::write_term;
use crate::json_file::ClassDataJson;
use crate::math::*;
use crate::matrices::Matrices;
use crate::parser::Gate;
use crate::parser::Instructions;
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

    /// row, col, val
    pub polys_px: Vec<Poly>,

    /// val, row, col
    pub points_px: Vec<HashMap<Mfp, Mfp>>,
}

impl Commitment {
    /// Constructor method Generate sets and Initilize matrices
    pub fn new(class_data: ClassDataJson) -> CommitmentBuilder {
        let numebr_t_zero = class_data.get_matrix_t_zeros() as u64;

        let set_h = generate_set(class_data.n, class_data);
        let set_k = generate_set(class_data.m, class_data);


        println_dbg!("$p: {}", P);
        println_dbg!("$g: {}", class_data.g);

        println_dbg!("set_h: {}", dsp_vec!(set_h));
        println_dbg!("set_k: {}", dsp_vec!(set_k));

        let matrix_size = class_data.get_matrix_size();
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
    pub fn get_polynomials_commitment(&self, commitment_key: &Vec<Mfp>) -> Vec<Mfp> {
        let commitment = compute_all_commitment(&self.polys_px, commitment_key);
        println_dbg!("com_ahp: {}", dsp_vec!(commitment));
        commitment
    }

    /// Store in Json file
    pub fn store(&self, path: &str, class_number: u8, class: ClassDataJson) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        let commitment_json = CommitmentJson::new(&self.polys_px, class_number, class);
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
    #[serde(rename = "commitmentID")]
    commitment_id: String,

    #[serde(rename = "IoT_Manufacturer_Name")]
    iot_manufacturer_name: String,

    #[serde(rename = "IoT_Device_Name")]
    iot_device_name: String,

    #[serde(rename = "Device_Hardware_Version")]
    device_hardware_version: String,

    #[serde(rename = "Firmware_Version")]
    firmware_version: String,

    #[serde(rename = "Class")]
    class: u8,

    m: u64,
    n: u64,
    p: u64,
    g: u64,

    #[serde(rename = "RowA")]
    row_a: Vec<u64>,

    #[serde(rename = "ColA")]
    col_a: Vec<u64>,

    #[serde(rename = "ValA")]
    val_a: Vec<u64>,

    #[serde(rename = "RowB")]
    row_b: Vec<u64>,

    #[serde(rename = "ColB")]
    col_b: Vec<u64>,

    #[serde(rename = "ValB")]
    val_b: Vec<u64>,

    #[serde(rename = "RowC")]
    row_c: Vec<u64>,

    #[serde(rename = "ColC")]
    col_c: Vec<u64>,

    #[serde(rename = "ValC")]
    val_c: Vec<u64>,

    #[serde(rename = "Curve")]
    curve: String,

    #[serde(rename = "PolynomialCommitment")]
    polynomial_commitment: String,
}

impl CommitmentJson {
    pub fn new(polys_px: &Vec<Poly>, class_number: u8, class: ClassDataJson) -> Self {
        // Extract values for CommitmentJson from the Commitment struct
        let polys_px_t: Vec<Vec<u64>> = polys_px.iter().map(|p| write_term(p)).collect();

        Self {
            commitment_id: "123456789".to_string(),
            iot_manufacturer_name: "FidesInnova".to_string(),
            iot_device_name: "test".to_string(),
            device_hardware_version: "1".to_string(),
            firmware_version: "2".to_string(),
            class: class_number,
            m: class.m,
            n: class.n,
            p: class.p,
            g: class.g,
            row_a: polys_px_t[0].clone(),
            col_a: polys_px_t[1].clone(),
            val_a: polys_px_t[2].clone(),
            row_b: polys_px_t[3].clone(),
            col_b: polys_px_t[4].clone(),
            val_b: polys_px_t[5].clone(),
            row_c: polys_px_t[6].clone(),
            col_c: polys_px_t[7].clone(),
            val_c: polys_px_t[8].clone(),
            curve: "bn128".to_string(),
            polynomial_commitment: "KZG".to_string(),
        }
    }

    fn convert_poly(v: &Vec<u64>) -> Poly {
        let mut poly = Poly::from(v.iter().rev().map(|&t| Mfp::from(t)).collect::<Vec<Mfp>>());
        poly.trim();
        poly
    }

    /// Retrieves the polynomial data as a vector of `Poly` instances.
    ///
    /// # Returns
    /// A vector of `Poly` objects constructed from the polynomial coefficients stored in `polys_px`.
    pub fn get_polys_px(&self) -> Vec<Poly> {
        vec![
            Self::convert_poly(&self.row_a),
            Self::convert_poly(&self.col_a),
            Self::convert_poly(&self.val_a),
            Self::convert_poly(&self.row_b),
            Self::convert_poly(&self.col_b),
            Self::convert_poly(&self.val_b),
            Self::convert_poly(&self.row_c),
            Self::convert_poly(&self.col_c),
            Self::convert_poly(&self.val_c),
        ]
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
    pub fn gen_matrices(&mut self, gates: Vec<Gate>, ni: usize) -> Self {
        // Initialize matrices A, B, C based on parsed gates
        let a_mat = &mut self.commitm.matrices.a;
        let b_mat = &mut self.commitm.matrices.b;
        let c_mat = &mut self.commitm.matrices.c;

        // Map the registers to last index
        let mut regs_data: HashMap<u8, usize> = HashMap::new();
        // let mut regs_data_right: HashMap<u8, usize> = HashMap::new();

        let mut _inx = 0;
        let mut counter = 0;
        // Left index
        let mut _li = 0;
        // Right index
        let mut _ri = 0;

        for (_, gate) in gates.iter().enumerate() {
            println_dbg!("Gate Loop: {} ------------", counter);
            
            // Set index
            _inx = 1 + ni + counter;

            // Update index
            (_li, _ri) = Self::get_register_index(&mut regs_data, gate.reg_left, gate.reg_right, gate.des_reg, _inx);

            // Get left and right values (index is zero if value exists)
            let left_val = Self::get_mfp_value(gate.val_left, &mut _li);
            let right_val = Self::get_mfp_value(gate.val_right, &mut _ri);

            println_dbg!("li: {_li}");
            println_dbg!("ri: {_ri}");

            c_mat[(_inx, _inx)] = Mfp::ONE;
            println_dbg!("C[{}, {}] = 1", _inx, _inx);


            match gate.instr {
                Instructions::Addi => {
                    println_dbg!("Gate: Add");
                    println_dbg!("A[{}, 0] = 1", _inx);
                    a_mat[(_inx, 0)] = Mfp::ONE;

                    println_dbg!("Left:  B[{}, {}] = {}", _inx, _li, left_val);
                    b_mat[(_inx, _li)] = left_val;

                    println_dbg!("Right: B[{}, {}] = {}", _inx, _ri, right_val);
                    b_mat[(_inx, _ri)] = right_val;
                }
                Instructions::Mul => {
                    println_dbg!("Gate: Mul");
                    println_dbg!("Left:  A[{}, {}] = {}", _inx, _li, left_val);
                    a_mat[(_inx, _li)] = left_val;

                    println_dbg!("Right: B[{}, {}] = {}", _inx, _ri, right_val);
                    b_mat[(_inx, _ri)] = right_val;
                }
                _ => panic!("Invalid gate {:?}", gate.instr),
            }
            counter += 1;
        }

        // Set specific rows in matrix C to zero
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

    fn get_register_index(
        regs_data: &mut HashMap<u8, usize>, 
        l_reg: u8, 
        r_reg: u8, 
        des: u8, 
        inx: usize
    ) -> (usize, usize) {
        println!("=>> {des} {l_reg} {r_reg}");
        
        let li = if regs_data.contains_key(&l_reg) {
            let inx_new = *regs_data.get(&l_reg).unwrap();
            regs_data.insert(des, inx);
            inx_new
        } else {
            l_reg as usize + 1
        };

        let ri = if regs_data.contains_key(&r_reg) {
            let inx_new = *regs_data.get(&r_reg).unwrap();
            regs_data.insert(des, inx);
            inx_new
        } else {
            r_reg as usize + 1
        };


        if !regs_data.contains_key(&des) {
            regs_data.insert(des, inx);
        }

        (li, ri)
    }

    /// Helper function to get Mfp value and index
    fn get_mfp_value(val: Option<u64>, index: &mut usize) -> Mfp {
        if let Some(v) = val {
            *index = 0; // Set index to zero if value exists
            println_dbg!("* index = 0, val = {}", v);
            Mfp::from(v)
        } else {
            println_dbg!("* index = {:<5}, val = None = 1", *index);
            Mfp::ONE
        }
    }

    pub fn add_val(
        regs_data: &mut HashMap<u8, RegData>,
        gate: &Gate,
        operator: Instructions,
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

    fn apply_operator(l: Mfp, r: Mfp, operator: Instructions) -> Mfp {
        match operator {
            Instructions::Addi => l + r,
            Instructions::Add => l + r,
            Instructions::Sub => l - r,
            Instructions::Mul => l * r,
            Instructions::Div => div_mod_val(l, r),
            Instructions::Ld => panic!("Invalid operation for Ld gate type"),
        }
    }

    pub fn gen_polynomials(&mut self) -> Self {
        let set_h = &self.commitm.set_h;
        let set_k = &self.commitm.set_k;

        // Collect row, column, and value points from matrix A
        let (points_row_p_a, points_col_p_a, points_val_p_a) =
            get_matrix_points(&self.commitm.matrices.a, set_h, set_k);
        // Collect row, column, and value points from matrix B
        let (points_row_p_b, points_col_p_b, points_val_p_b) =
            get_matrix_points(&self.commitm.matrices.b, set_h, set_k);
        // Collect row, column, and value points from matrix C.
        let (points_row_p_c, points_col_p_c, points_val_p_c) =
            get_matrix_points(&self.commitm.matrices.c, set_h, set_k);

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
