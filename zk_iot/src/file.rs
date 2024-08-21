//! Utilities for storing polynomials and sets in JSON files.

use std::{fs::{self, File, OpenOptions}, io::{BufReader, Write}, path::PathBuf};
use crate::{math::{Mfp, Poly}, to_bint};
use rustnomial::SizedPolynomial;
use serde_json::{json, Value};
use anyhow::{Result, anyhow};


// Path to the JSON file used for storing data.
const JSON_PATH: &str = "data.json";


/// Creates a new, empty file at the specified JSON path.
/// 
/// # Returns
/// Returns an `io::Result<()>` indicating the success or failure of the file creation.
pub fn clean_file() -> Result<()> {
    File::create(JSON_PATH)?;
    Ok(())
}


/// Converts a polynomial to a string representation of its terms.
/// 
/// # Parameters
/// - `poly`: A reference to a `Poly` object whose terms are to be converted to a string.
///
/// # Returns
/// Returns a `String` containing the terms of the polynomial, where each term is formatted as `"(coefficient, exponent)"`.
fn write_term(poly: &Poly) -> String {
    let mut poly = poly.clone();
    poly.trim();
    poly.terms_as_vec().iter().map(|v| format!("({},{})", to_bint!(v.0), v.1)).collect::<String>()
}


/// Converts a set of `Mfp` elements to a string.
/// 
/// # Parameters
/// - `set`: A slice of `Mfp` elements to be converted to a string.
///
/// # Returns
/// Returns a `String` containing the elements of the set.
fn write_set(set: &[Mfp]) -> String {
    set.iter().map(|v| format!("{} ", to_bint!(*v))).collect::<String>()
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
fn add_value_to_json_file(value: Value) -> Result<()> {
    // Check if the file exists
    let mut data = if fs::metadata(JSON_PATH).is_ok() {
        let file_content = fs::read_to_string(JSON_PATH)?;
        serde_json::from_str::<Value>(&file_content).unwrap_or(Value::Object(serde_json::Map::new()))
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Merge the new value into the existing data
    if let Value::Object(ref mut existing_map) = data {
        if let Value::Object(new_map) = value {
            for (key, val) in new_map {
                existing_map.insert(key, val);
            }
        }
    }

    // Write the updated data back to the file
    let json_string = serde_json::to_string_pretty(&data)?;
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(JSON_PATH)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

/// Stores a polynomial in JSON format under a specified name.
///
/// # Parameters
/// - `name`: A string slice representing the name under which the polynomial is stored.
/// - `poly`: A reference to the `Poly` object to be stored.
///
/// # Returns
/// Returns a `Result<()>`, indicating success or failure in storing the polynomial in the JSON file.
pub fn store_poly_json(name: &str, poly: &Poly) -> Result<()> {
    let poly = json!(write_term(poly));

    let json_value = json!({
        format!("{}", name): poly,
    });

    add_value_to_json_file(json_value)
}



/// Stores a set of `Mfp` values in JSON format under a specified name.
///
/// # Parameters
/// - `name`: A string slice representing the name under which the set is stored.
/// - `set`: A slice of `Mfp` values to be stored.
///
/// # Returns
/// Returns a `Result<()>`, indicating success or failure in storing the set in the JSON file.
pub fn store_set_json(name: &str, set: &[Mfp]) -> Result<()> {
    let poly = json!(write_set(set));

    let json_value = json!({
        format!("{}", name): poly,
    });

    add_value_to_json_file(json_value)
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
pub fn store_commit_json(polys: &[&Poly]) -> Result<()> {
    if polys.len() != 9 {
        return Err(anyhow!("Insufficient number of polynomials"));
    }
    let a_array = vec![json!(write_term(polys[0])), json!(write_term(polys[1])), json!(write_term(polys[2]))];
    let b_array = vec![json!(write_term(polys[3])), json!(write_term(polys[4])), json!(write_term(polys[5]))];
    let c_array = vec![json!(write_term(polys[6])), json!(write_term(polys[7])), json!(write_term(polys[8]))];

    let json_value = json!({
        "A": a_array,
        "B": b_array,
        "C": c_array,
    });

    add_value_to_json_file(json_value)
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
