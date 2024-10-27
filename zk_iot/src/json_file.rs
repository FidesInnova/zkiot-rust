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

use std::{collections::HashMap, fs::{File, OpenOptions}, io::{BufReader, Write}, path::PathBuf};
use crate::{math::{Mfp, Poly}, to_bint};
use ark_ff::Field;
use rustnomial::{Degree, SizedPolynomial};
use serde::Deserialize;
use serde_json::Value;
use anyhow::{anyhow, Result};


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
        let index = poly_terms.iter().position(|v| v.1 == i).unwrap_or(usize::MAX);
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
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
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
    set.iter().map(|v| to_bint!(*v) as u64).collect::<Vec<u64>>()
}


pub fn read_term(poly: &[Value]) -> Poly {
    let poly_vec = poly.to_vec().iter().rev().map(|v| Mfp::from(v.as_u64().unwrap())).collect::<Vec<Mfp>>();
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
pub struct ClassData {
    pub n_g: u64,
    pub n_i: u64,
    pub n: u64,
    pub m: u64,
    row_a: u64,
    col_a: u64,
    val_a: u64,
    row_b: u64,
    col_b: u64,
    val_b: u64,
    row_c: u64,
    col_c: u64,
    val_c: u64,
}

pub fn get_class_data(path: &str, class_type: &str) -> Result<ClassData> {
    let reader = open_file(&PathBuf::from(path))?;
    // Deserialize the JSON into a HashMap
    let data: HashMap<String, ClassData> = serde_json::from_reader(reader)?;
    let class_to_access = class_type;
    if let Some(class_data) = data.get(class_to_access) {
        Ok(class_data.clone())
    } else {
        Err(anyhow!("Class {} doesn't exist", class_to_access))
    }
}
