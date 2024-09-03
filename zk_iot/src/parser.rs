//! Module for parsing gate information from text files into `Gate` objects.

use anyhow::{anyhow, Context, Result};
use std::{fs::File, path::PathBuf};
use std::io::{BufRead, BufReader};

use crate::math::Poly;
use crate::utils::{Gate, GateType};
use crate::json_file::*;

/// Parses a line of text into a tuple containing a specific element and a vector of elements.
///
/// # Parameters
/// - `line`: A string slice representing the line of text to be parsed.
/// - `index`: The line index used for error reporting.
///
/// # Returns
/// - `Ok`: Returns a tuple where the first element is a string slice (`&str`) corresponding to the third part of the line, and the second element is a vector of string slices (`Vec<&str>`) containing the remaining parts from the fourth onward.
/// - `Err`: Returns an error if the line does not contain at least six parts, with an error message including the line index.
///
/// # Errors
/// Returns an error if the line does not contain at least six non-empty parts, as determined by splitting on commas and spaces and filtering out empty parts.
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

pub fn parse_from_lines(line_file: BufReader<File>, opcodes_file: &PathBuf) -> Result<Vec<Gate>> {
    let mut gates = Vec::new();

    for (index, line) in line_file.lines().enumerate() {
        let line_num = line
            .context(format!("Error reading line {}: unable to parse line number", index + 1))?
            .trim()
            .parse::<usize>()
            .context(format!("Error parsing line number from line {}", index + 1))?;

        let gates_file = open_file(opcodes_file).context("Failed to open opcodes file")?;
        let line = gates_file
            .lines()
            .nth(line_num - 1)
            .ok_or_else(|| anyhow!("Line number {} is out of bounds in opcodes file", line_num))??;

        let (operation, operands) = parse_line(&line, line_num)
            .context(format!("Error parsing line {}: {}", line_num, line))?;
        
        let gate_type = gate_type(operation);
        if let Err(ref e) = gate_type {
            // Return Err
            eprintln!("Error determining gate type for line {}: {}", line_num, e);
            continue;
        }

        let constant = operands.get(2)
            .ok_or_else(|| anyhow!("Missing operand at index 2 for line {}", line_num))?
            .parse::<u64>()
            .context(format!("Error parsing constant from operands for line {}", line_num))?;

        let gate = Gate::new(line_num, 0, None, Some(constant), gate_type?);
        gates.push(gate);
    }

    Ok(gates)
}

/// Parses a file to generate a vector of `Gate` objects.
///
/// # Parameters
/// - `file_path`: A `PathBuf` representing the path to the file that contains the gate data.
///
/// # Returns
/// - `Ok(Vec<Gate>)`: A vector of `Gate` objects parsed from the file.
/// - `Err(anyhow::Error)`: An error if there is an issue opening or reading the file.
///
/// # Description
/// The `parser` function reads the file located at `file_path`, parses its content into `Gate` objects,
/// and returns the resulting vector. It utilizes the `open_file` function to handle the file reading
/// and then delegates parsing to `read_parse_lines`.
pub fn parser(file_path: PathBuf) -> Result<Vec<Gate>> {
    let reader = open_file(&file_path)?;
    let gates = read_parse_lines(reader)?;
    Ok(gates)
}

/// Parses gates from a file containing line numbers and a corresponding opcodes file.
///
/// # Parameters
/// - `line_file`: A reference to a `PathBuf` representing the path to the file with line numbers.
/// - `opcodes_file`: A reference to a `PathBuf` representing the path to the file with opcodes.
///
/// # Returns
/// - `Ok(Vec<Gate>)`: A vector of `Gate` objects if parsing is successful.
/// - `Err(anyhow::Error)`: An error if there is an issue reading or parsing the files.
///
/// # Description
/// This function processes each line in the `line_file`, treating the content as line numbers that
/// reference specific lines in the `opcodes_file`. For each valid line, it parses the corresponding
/// opcode line to determine the operation and operands, constructs a `Gate` object, and adds it to
/// the resulting vector. Errors are handled and reported with contextual information.
fn read_parse_lines(reader: BufReader<File>) -> Result<Vec<Gate>> {
    let mut gates = Vec::new();

    for (index, line_result) in reader.lines().enumerate() {
        let line = line_result
            .context(format!("Error reading line {}: unable to read line", index + 1))?;
        
        let (operation, operands) = parse_line(&line, index)
            .context(format!("Error parsing line {}: {}", index + 1, line))?;
        
        let gate_type = gate_type(operation);
        if let Err(ref e) = gate_type {
            eprintln!("Error determining gate type for line {}: {}", index + 1, e);
            continue;
        }

        let constant = operands.get(2)
            .ok_or_else(|| anyhow!("Missing operand at index 2 for line {}", index + 1))?
            .parse::<u64>()
            .context(format!("Error parsing constant from operands for line {}", index + 1))?;

        let gate = Gate::new(index + 1, 0, None, Some(constant), gate_type?);
        gates.push(gate);
    }

    Ok(gates)
}

/// Determines the `GateType` based on the given operation string.
///
/// # Parameters
/// - `op`: A reference to a string slice representing the operation to be parsed.
///
/// # Returns
/// - `Ok(GateType)`: The corresponding `GateType` if the operation is recognized.
/// - `Err(anyhow::Error)`: An error if the operation is not supported.
///
/// # Description
/// The `gate_type` function matches the input operation string to predefined gate types. 
/// If the operation is recognized (e.g., `"mul"` or `"addi"`), the corresponding `GateType` 
/// is returned. If the operation is unrecognized, the function returns an error indicating 
/// that the operation is not supported.
fn gate_type(op: &str) -> Result<GateType> {
    match op {
        "mul" => Ok(GateType::Mul),
        "addi" => Ok(GateType::Add),
        _ => Err(anyhow!("operation is not support: {}", op)),
    }
}


#[cfg(test)]
mod parser_test {
    use super::*;

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
