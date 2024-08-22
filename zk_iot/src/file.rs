//! Utilities for storing polynomials and sets in JSON files.

use std::{collections::HashMap, fs::{self, File, OpenOptions}, io::{BufReader, Write}, path::PathBuf};
use crate::{dsp_vec, math::{Mfp, Poly}, to_bint};
use ark_ff::Field;
use rustnomial::{Degree, SizedPolynomial};
use serde_json::{json, Value};
use anyhow::{Result, anyhow};


// Path to the JSON file used for storing data.
pub const JSON_COMMIT_PATH: &str = "commit.json";
pub const JSON_PROOF_PATH: &str  = "proof.json";


/// Creates a new, empty files at the specified JSON path.
/// 
/// # Returns
/// Returns an `io::Result<()>` indicating the success or failure of the file creation.
pub fn clean_files() -> Result<()> {
    File::create(JSON_COMMIT_PATH)?;
    File::create(JSON_PROOF_PATH)?;
    Ok(())
}

/// Converts a polynomial to a string representation of its terms.
/// 
/// # Parameters
/// - `poly`: A reference to a `Poly` object whose terms are to be converted to a string.
///
/// # Returns
/// Returns a `String` containing the terms of the polynomial, where each term is formatted as `"(coefficient, exponent)"`.
fn write_term(poly: &Poly, max_deg: usize) -> Vec<u64> {
    let mut poly = poly.clone();
    poly.trim();
    let poly_mapped = poly.terms_as_vec().iter().map(|v| (v.1, to_bint!(v.0))).collect::<HashMap<usize, u64>>();
    let mut poly = vec![0; max_deg];
    for i in 0..poly.len() {
        poly[i] = *poly_mapped.get(&i).unwrap_or(&0);
    }
    // dsp_vec!(poly)
    poly
}

/// Adds a new JSON value to an existing JSON file, merging it with any existing data.
///
/// # Parameters
/// - `value`: A `Value` object representing the JSON data to be added.
///
/// # Returns
/// Returns a `Result<()>`, indicating success or failure in adding the value to the JSON file.
/// 
/// # Details
/// - If the file already exists, it reads the content and merges the new value into the existing data.
/// - If the file does not exist or is empty, it creates a new JSON object with the provided value.
/// - The updated data is then written back to the file in a pretty-printed format.
fn add_value_to_json_file(value: Value, path: &str) -> Result<()> {
    let json_string = serde_json::to_string(&value)?;
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

/// Stores the commitment polynomials in JSON format.
///
/// # Parameters
/// - `polys`: A slice containing references to 9 polynomials, representing 
///   the `A`, `B`, and `C` arrays.
///
/// # Returns
/// Returns a `Result<()>`, indicating success or failure in storing the commitment polynomials.
/// If the slice does not contain exactly 9 polynomials, an error is returned.
pub fn store_commit_json(polys: &[&Poly], t: usize, n: usize) -> Result<()> {
    let m = (((n * n) - n) / 2) + (((t * t) - t) / 2);

    let json_value = json!({
        "n": n,
        "t": t,
        "p1": write_term(polys[0], m),
        "p2": write_term(polys[1], m),
        "p3": write_term(polys[2], m),
        "p4": write_term(polys[0], m),
        "p5": write_term(polys[1], m),
        "p6": write_term(polys[2], m),
        "p7": write_term(polys[0], m),
        "p8": write_term(polys[1], m),
        "p9": write_term(polys[2], m),
    });

    add_value_to_json_file(json_value, JSON_COMMIT_PATH)
}

pub fn store_proof_json(polys: &[&Poly], sigma: &[&Mfp], b: usize, set_h_len: usize, set_k_len: usize) -> Result<()> {
    let poly_0_size = if let Degree::Num(num) = polys[0].degree() {
        num
    } else {
        0
    };

    let json_value = json!({
        "p1": to_bint!(*sigma[0]), // sigma_1
        "p2": write_term(polys[0], poly_0_size + b - 1), // w^x 
        "p3": write_term(polys[1], set_h_len + b - 1),  // z^a
        "p4": write_term(polys[2], set_h_len + b - 1),  // z^b
        "p5": write_term(polys[3], set_h_len + b - 1),  // z^b
        "p6": write_term(polys[4], set_h_len + 2 * b - 1),  // h_0
        "p7": write_term(polys[5], set_h_len + b - 2),  // sx
        "p8": write_term(polys[6], set_h_len - 2),      // g_1
        "p9": write_term(polys[7], set_h_len + b - 2),  // h_1
        "p10": to_bint!(*sigma[1]), // sigma2
        "p11": write_term(polys[8], set_h_len - 2),  // g_2
        "p12": write_term(polys[9], set_h_len - 2),  // h_2
        "p13": to_bint!(*sigma[2]) // sigma3
        // p14 = g_3
        // p15 = h_3
    });

    add_value_to_json_file(json_value, JSON_PROOF_PATH)
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
