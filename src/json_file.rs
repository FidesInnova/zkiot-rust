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

//! Utilities for storing polynomials and sets in JSON files.
use anyhow::anyhow;
use anyhow::Result;
use ark_ff::Field;
use nalgebra::DMatrix;
use rustnomial::Degree;
use rustnomial::SizedPolynomial;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use crate::math::generate_set;
use crate::math::Mfp;
use crate::math::Poly;
use crate::matrices;
use crate::matrices::Matrices;
use crate::to_bint;
use crate::utils::read_json_file;

/// Converts a polynomial to a vector representation of its coefficients.
///
/// # Parameters
/// - `poly`: A reference to a `Poly` object whose terms are to be converted to a vector of coefficients.
/// - `max_deg`: The maximum degree of the polynomial, which determines the size of the returned vector.
///
/// # Returns
/// Returns a `Vec<u64>` containing the coefficients of the polynomial, where the index represents the exponent
/// of each term. If a term does not exist for a particular exponent, the coefficient at that index will be `0`.
pub fn write_term(poly: &Poly) -> Vec<u64> {
    let mut poly = poly.clone();
    poly.trim();

    // let poly_mapped = poly.terms_as_vec().iter().map(|v| (v.1, to_bint!(v.0))).collect::<HashMap<usize, u64>>();

    let poly_terms = poly.terms_as_vec();

    let max_deg = if let Degree::Num(n) = poly.degree() {
        n
    } else {
        0
    };

    let mut poly = vec![0; max_deg + 1];

    for (i, poly) in poly.iter_mut().enumerate().take(max_deg + 1) {
        let index = poly_terms
            .iter()
            .position(|v| v.1 == i)
            .unwrap_or(usize::MAX);
        *poly = to_bint!(poly_terms.get(index).unwrap_or(&(Mfp::ZERO, 0)).0);
    }

    poly
}
/// Adds a new JSON value to an existing JSON file, replacing any existing data.
///
/// # Parameters
/// - `value`: A `Value` object representing the JSON data to be added.
/// - `path`: A string slice representing the path to the JSON file.
///
/// # Returns
/// Returns a `Result<()>`, indicating success or failure in adding the value to the JSON file.
///
/// # Details
/// - If the file already exists, it will be truncated, and the new value will replace any existing data.
/// - If the file does not exist, it creates a new JSON file with the provided value.
/// - The updated data is then written back to the file in a compact format.
pub fn store_in_json_file(value: Value, path: &str) -> Result<()> {
    let json_string = serde_json::to_string(&value)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}

/// Converts a vector of `Mfp` objects to a vector of `u64` values.
///
/// # Parameters
/// - `set`: A reference to a vector containing `Mfp` objects to be converted.
///
/// # Returns
/// Returns a `Vec<u64>` containing the converted values, where each `Mfp` object
/// is transformed into a `u64` representation using the `to_bint!` macro.
pub fn write_set(set: &Vec<Mfp>) -> Vec<u64> {
    set.iter()
        .map(|v| to_bint!(*v) as u64)
        .collect::<Vec<u64>>()
}

pub fn read_term(poly: &[Value]) -> Poly {
    let poly_vec = poly
        .to_vec()
        .iter()
        .rev()
        .map(|v| Mfp::from(v.as_u64().unwrap()))
        .collect::<Vec<Mfp>>();
    let mut poly = Poly::from(poly_vec);
    poly.trim();
    poly
}

/// Opens a file and returns a buffered reader.
///
/// # Parameters
/// - `file_path`: A reference to a `PathBuf` representing the path to the file.
///
/// # Returns
/// - `Ok(BufReader<File>)`: A buffered reader for the opened file.
/// - `Err(anyhow::Error)`: An error if there is an issue opening the file.
///
/// # Description
/// The `open_file` function attempts to open the file at the specified path and returns
/// a `BufReader` to allow for efficient reading of the file's contents.
pub fn open_file(file_path: &PathBuf) -> Result<BufReader<File>> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file))
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct ClassDataJson {
    /// Number of gates
    pub n_g: u64,
    /// Number of inputs
    pub n_i: u64,
    /// N = n_i + n_g
    pub n: u64,
    /// M = 2 * n_g
    pub m: u64,
    /// Prime number
    pub p: u64,
    /// Generator
    pub g: u64,
}

impl ClassDataJson {
    pub fn get_class_data(path: &str, class_type: u8) -> Result<ClassDataJson> {
        // Retrieve all class data from the specified path
        let data = Self::get_all_class_data(path)?;

        // Specify the class type to access
        let class_to_access = class_type;

        // Return the specified class data if it exists
        if let Some(class_data) = data.get(&class_to_access) {
            Ok(class_data.clone())
        } else {
            Err(anyhow!("Class {} doesn't exist", class_to_access))
        }
    }

    /// Returns the size of the matrix based on class data
    pub fn get_matrix_size(&self) -> usize {
        (self.n_g + self.n_i + 1).try_into().unwrap()
    }

    /// Returns the number of zero rows in the matrix based on class data
    pub fn get_matrix_t_zeros(&self) -> usize {
        // Number of rows (|x| = numebr_t_zero, where numebr_t_zero = ni + 1)
        (self.n_i + 1).try_into().unwrap()
    }

    pub fn get_all_class_data(path: &str) -> Result<HashMap<u8, ClassDataJson>> {
        let reader = open_file(&PathBuf::from(path))?;
        // Deserialize the JSON into a HashMap
        let data: HashMap<u8, ClassDataJson> = serde_json::from_reader(reader)?;
        Ok(data)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProgramParamsJson {
    /// [..t_zeros skipped.., col1, col2, col3, ...]
    #[serde(rename = "A")]
    a: Vec<u64>,

    /// [(row, col, val), ...]
    #[serde(rename = "B")]
    b: Vec<(usize, usize, u64)>,

    /// Values of ranges: [[point_val_a, point_col_a, point_row_a, ...]]
    points_px: Vec<Vec<u64>>,
}

impl ProgramParamsJson {
    pub fn new(
        matrices: &Matrices,
        points_px: &Vec<HashMap<Mfp, Mfp>>,
        class_data: ClassDataJson,
    ) -> Self {
        // store points accordint to set_k
        let set_k = generate_set(class_data.m, class_data);
        Self {
            a: Matrices::to_sparse_column_indices(&matrices.a, class_data.get_matrix_t_zeros()),
            b: Matrices::to_sparse_coordinate_form(&matrices.b),
            points_px: Self::to_points_u64(points_px, &set_k),
        }
    }

    #[allow(warnings)]
    fn to_points_u64(points_px: &Vec<HashMap<Mfp, Mfp>>, set_k: &Vec<Mfp>) -> Vec<Vec<u64>> {
        let mut points_px_t: Vec<Vec<(u64, u64)>> = points_px
            .iter()
            .map(|points| {
                points
                    .iter()
                    .map(|(&key, &val)| (to_bint!(key), to_bint!(val)))
                    .collect::<Vec<(u64, u64)>>()
            })
            .collect();

        // Sort each set of points according to their corresponding indices in set_k
        for points in &mut points_px_t {
            points.sort_by_key(|&(x, _)| {
                // Assuming x corresponds to the key in set_k
                // Find the index of x in set_k to use as the sorting key
                set_k
                    .iter()
                    .position(|&k| to_bint!(k) == x)
                    .unwrap_or(usize::MAX)
            });
        }

        // Extract the second element (val) from each tuple and return
        points_px_t
            .iter()
            .map(|points| {
                points
                    .iter()
                    .map(|(_, val)| *val)
                    .collect::<Vec<u64>>()
            })
            .collect()
    }

    fn get_matrix_a(&self, size: usize, number_t_zeros: usize) -> DMatrix<Mfp> {
        let mut mat_a = DMatrix::<Mfp>::zeros(size, size);

        for (i, &j) in self.a.iter().enumerate() {
            mat_a[(i + number_t_zeros, j.try_into().unwrap())] = Mfp::ONE;
        }

        mat_a
    }

    fn get_matrix_b(&self, size: usize) -> DMatrix<Mfp> {
        let mut mat_b = DMatrix::<Mfp>::zeros(size, size);

        for &(i, j, val) in self.b.iter() {
            mat_b[(i, j)] = Mfp::from(val);
        }

        mat_b
    }

    /// Retrieves the points data as a vector of hash maps.
    ///
    /// # Returns
    /// A vector of hash maps where each map represents a set of points with `Mfp` keys and values.
    pub fn get_points_px(&self, set_k: &Vec<Mfp>) -> Vec<HashMap<Mfp, Mfp>> {
        self.points_px
            .iter()
            .map(|points| {
                points
                    .iter()
                    .enumerate()
                    .map(|(i, &p)| (set_k[i], Mfp::from(p)))
                    .collect()
            })
            .collect()
    }

    /// Retrieves matrices A, B, and C based on the provided matrices JSON and class data.
    ///
    /// # Parameters
    /// - `matrices`: A reference to a `MatricesJson` object containing matrix data.
    /// - `class_data`: A reference to a `ClassData` object used to determine the size of the matrices.
    ///
    /// # Returns
    /// A tuple containing three dense matrices: (A, B, C).
    pub fn get_matrices(
        &self,
        class_data: &ClassDataJson,
    ) -> (DMatrix<Mfp>, DMatrix<Mfp>, DMatrix<Mfp>) {
        let a = self.get_matrix_a(
            class_data.get_matrix_size(),
            class_data.get_matrix_t_zeros(),
        );
        let b = self.get_matrix_b(class_data.get_matrix_size());
        let c = Matrices::generate_matrix_c(
            class_data.get_matrix_size(),
            class_data.get_matrix_t_zeros(),
        );

        (a, b, c)
    }

    /// Store in Json file
    pub fn store(&self, path: &str) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    /// Restore Commitment from Json file
    pub fn restore(path: &str) -> Result<Self> {
        read_json_file(path)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(untagged)]
pub enum LineValue {
    Range((usize, usize)),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeviceConfigJson {
    pub class: u8,
    pub iot_manufacturer_name: String,
    pub iot_device_name: String,
    pub device_hardware_version: String,
    pub firmware_version: String,
    pub code_block: LineValue,
}

impl DeviceConfigJson {
    pub fn convert_lines(lines: LineValue) -> Vec<usize> {
        let LineValue::Range(r) = lines;
        (r.0..=r.1).collect()
    }
}
