//! Utilities for storing polynomials and sets in JSON files.

use std::{collections::HashMap, fs::{self, File, OpenOptions}, io::{BufReader, Write}, path::PathBuf};
use crate::{dsp_vec, math::{Mfp, Poly}, to_bint};
use ark_ff::Field;
use rustnomial::{Degree, SizedPolynomial};
use serde_json::{json, Value};
use anyhow::{anyhow, Context, Result};


// Path to the JSON file used for storing data.
pub const JSON_COMMIT_PATH: &str = "commit.json";
pub const JSON_PROOF_PATH: &str  = "proof.json";

/// Converts a polynomial to a vector representation of its coefficients.
/// 
/// # Parameters
/// - `poly`: A reference to a `Poly` object whose terms are to be converted to a vector of coefficients.
/// - `max_deg`: The maximum degree of the polynomial, which determines the size of the returned vector.
///
/// # Returns
/// Returns a `Vec<u64>` containing the coefficients of the polynomial, where the index represents the exponent 
/// of each term. If a term does not exist for a particular exponent, the coefficient at that index will be `0`.
fn write_term(poly: &Poly) -> Vec<u64> {
    let mut poly = poly.clone();
    poly.trim();

    // let poly_mapped = poly.terms_as_vec().iter().map(|v| (v.1, to_bint!(v.0))).collect::<HashMap<usize, u64>>();

    let mut poly_terms = poly.terms_as_vec();

    let max_deg = if let Degree::Num(n) = poly.degree() {
        n
    } else {
        0
    };

    let mut poly = vec![0; max_deg + 1];

    for i in 0..=max_deg {
        let index = poly_terms.iter().position(|v| v.1 == i).unwrap_or(usize::MAX);
        poly[i] = to_bint!(poly_terms.get(index).unwrap_or(&(Mfp::ZERO, 0)).0);
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
fn add_value_to_json_file(value: Value, path: &str) -> Result<()> {
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
fn write_set(set: &Vec<Mfp>) -> Vec<u64> {
    set.iter().map(|v| to_bint!(*v) as u64).collect::<Vec<u64>>()
}

/// Stores the commitment polynomials in JSON format.
///
/// # Parameters
/// - `polys`: A slice containing references to 9 polynomials, representing 
///   the `A`, `B`, and `C` arrays.
/// - `t`: An integer representing a parameter related to the commitment scheme.
/// - `n`: An integer representing the size parameter for the polynomials.
/// - `pp`: A vector of `Mfp` objects representing the proof path.
///
/// # Returns
/// Returns a `Result<()>`, indicating success or failure in storing the commitment polynomials.
/// If the slice does not contain exactly 9 polynomials, an error is returned.
pub fn store_commit_json(polys: &[&Poly], m: usize, n: usize, sets: [Vec<Mfp>; 2]) -> Result<()> {
    let json_value = json!({
        "m": m,
        "n": n,
        "set_h": write_set(&sets[0]),
        "set_k": write_set(&sets[1]),
        "ComRowA": write_term(polys[0]),
        "ComColA": write_term(polys[1]),
        "ComValA": write_term(polys[2]),
        "ComRowB": write_term(polys[3]),
        "ComColB": write_term(polys[4]),
        "ComValB": write_term(polys[5]),
        "ComRowC": write_term(polys[6]),
        "ComColC": write_term(polys[7]),
        "ComValC": write_term(polys[8]),
        "Curve": "bn128",
        "PolynomialCommitment": "KZG"
    });

    add_value_to_json_file(json_value, JSON_COMMIT_PATH)
}

fn read_term(poly: &[Value]) -> Poly {
    let poly_vec = poly.to_vec().iter().rev().map(|v| Mfp::from(v.as_u64().unwrap())).collect::<Vec<Mfp>>();
    let mut poly = Poly::from(poly_vec);
    poly.trim();
    poly
}

pub fn restore_commit_json(json_path: &str) -> Result<(Vec<Poly>, usize, usize, Vec<Mfp>, [Vec<Mfp>; 2])> {
    let file_content = fs::read_to_string(json_path)?;
    let json_value: Value = serde_json::from_str(&file_content)?;

    let proof_path = json_value["ProofPath"].as_array().unwrap().to_vec()
                .iter().map(|v| Mfp::from(v.as_u64().unwrap())).collect::<Vec<Mfp>>();
    let n = json_value["n"].as_u64().unwrap() as usize;
    let t = json_value["t"].as_u64().unwrap() as usize;
    let set_h = json_value["set_h"].as_array().unwrap().to_vec()
    .iter().map(|v| Mfp::from(v.as_u64().unwrap())).collect::<Vec<Mfp>>();
    let set_k = json_value["set_k"].as_array().unwrap().to_vec()
    .iter().map(|v| Mfp::from(v.as_u64().unwrap())).collect::<Vec<Mfp>>();

    let polys = vec![
        read_term(json_value["ComRowA"].as_array().unwrap()),
        read_term(json_value["ComColA"].as_array().unwrap()),
        read_term(json_value["ComValA"].as_array().unwrap()),
        read_term(json_value["ComRowB"].as_array().unwrap()),
        read_term(json_value["ComColB"].as_array().unwrap()),
        read_term(json_value["ComValB"].as_array().unwrap()),
        read_term(json_value["ComRowC"].as_array().unwrap()),
        read_term(json_value["ComColC"].as_array().unwrap()),
        read_term(json_value["ComValC"].as_array().unwrap()),
    ];

    Ok((polys, t, n, proof_path, [set_h, set_k]))
}


/// Stores proof data in JSON format.
///
/// # Parameters
/// - `polys`: A slice containing references to polynomials used in the proof.
/// - `sigma`: A slice containing references to `Mfp` objects representing sigma values.
/// - `b`: An integer representing a parameter related to the proof structure.
/// - `set_h_len`: An integer representing the length of the set for `h` values.
/// - `set_k_len`: An integer representing the length of the set for `k` values.
///
/// # Returns
/// Returns a `Result<()>`, indicating success or failure in storing the proof data.
/// If the input data is not valid, an error may be returned.
pub fn store_proof_json(polys: &[&Poly], sigma: &[&Mfp], b: usize, set_h_len: usize, set_k_len: usize) -> Result<()> {
    let poly_0_size = if let Degree::Num(num) = polys[0].degree() {
        num
    } else {
        0
    };

    let json_value = json!({
        "P1AHP": to_bint!(*sigma[0]), // sigma_1
        "P2AHP": write_term(polys[0]), // w^x 
        "P3AHP": write_term(polys[1]),  // z^a
        "P4AHP": write_term(polys[2]),  // z^b
        "P5AHP": write_term(polys[3]),  // z^b
        "P6AHP": write_term(polys[4]),  // z^x 
        "P7AHP": write_term(polys[5]),  // h_0
        "P8AHP": write_term(polys[6]),  // sx
        "P9AHP": write_term(polys[7]),      // g_1
        "P10AHP": write_term(polys[8]),  // h_1
        "P11AHP": to_bint!(*sigma[1]), // sigma2
        "P12AHP": write_term(polys[9]),  // g_2
        "P13AHP": write_term(polys[10]),  // h_2
        "P14AHP": to_bint!(*sigma[2]), // sigma3
        "P15AHP": write_term(polys[11]), 
        "P16AHP": write_term(polys[12]),
        "protocol":"fidesv1",
        "curve": "bn128"
    });

    add_value_to_json_file(json_value, JSON_PROOF_PATH)
}


pub fn restore_proof_json(json_path: &str) -> Result<(Vec<Poly>, Vec<Mfp>)> {
    let file_content = fs::read_to_string(json_path)?;
    let json_value: Value = serde_json::from_str(&file_content)?;

    let polys = vec![
        read_term(json_value["P2AHP"].as_array().unwrap()),
        read_term(json_value["P3AHP"].as_array().unwrap()),
        read_term(json_value["P4AHP"].as_array().unwrap()),
        read_term(json_value["P5AHP"].as_array().unwrap()),
        read_term(json_value["P6AHP"].as_array().unwrap()),
        read_term(json_value["P7AHP"].as_array().unwrap()),
        read_term(json_value["P8AHP"].as_array().unwrap()),
        read_term(json_value["P9AHP"].as_array().unwrap()),
        read_term(json_value["P11AHP"].as_array().unwrap()),
        read_term(json_value["P12AHP"].as_array().unwrap()),
        read_term(json_value["P14AHP"].as_array().unwrap()),
        read_term(json_value["P15AHP"].as_array().unwrap()),
    ];

    let sigma = vec![
        Mfp::from(json_value["P1AHP"].as_u64().unwrap()),
        Mfp::from(json_value["P10AHP"].as_u64().unwrap()),
        Mfp::from(json_value["P13AHP"].as_u64().unwrap()),
    ];

    Ok((polys, sigma))
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
