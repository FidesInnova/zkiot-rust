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


//! Module for parsing gate information from text files into `Gate` objects.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::{fs::File, path::PathBuf};
use crate::{json_file::*, println_dbg};
use crate::math::Mfp;




#[derive(Debug)]
enum ZKPRegister {
    X0,  // Hardwired zero
    X1,  // Return address (ra)
    X2,  // Stack pointer (sp)
    X3,  // Global pointer (gp)
    X4,  // Thread pointer (tp)
    X5,  // Temporary register (t0)
    X6,  // Temporary register (t1)
    X7,  // Temporary register (t2)
    X8,  // Platform register (s0)
    X9,  // Platform register (s1)
    X10, // Argument register (a0)
    X11, // Argument register (a1)
    X12, // Temporary register (a2)
    X13, // Temporary register (a3)
    X14, // Temporary register (a4)
    X15, // Temporary register (a5)
    X16, // Temporary register (a6)
    X17, // Temporary register (a7)
    X18, // Saved register (s2)
    X19, // Saved register (s3)
    X20, // Saved register (s4)
    X21, // Saved register (s5)
    X22, // Saved register (s6)
    X23, // Saved register (s7)
    X24, // Saved register (s8)
    X25, // Saved register (s9)
    X26, // Saved register (s10)
    X27, // Saved register (s11)
    X28, // Temporary register (t3)
    X29, // Frame pointer (t4)
    X30, // Return address (t5)
    X31, // Integer register (t6)
}

#[derive(Debug)]
enum RiscvReg {
    Zero, // x0 - Hardwired zero
    Ra,   // x1 - Return address
    Sp,   // x2 - Stack pointer
    Gp,   // x3 - Global pointer
    Tp,   // x4 - Thread pointer
    T0,   // x5 - Temporary register
    T1,   // x6 - Temporary register
    T2,   // x7 - Temporary register
    S0,   // x8 - Saved register
    S1,   // x9 - Saved register
    A0,   // x10 - Argument register
    A1,   // x11 - Argument register
    T3,   // x12 - Temporary register
    T4,   // x13 - Temporary register
    T5,   // x14 - Temporary register
    T6,   // x15 - Temporary register
    A2,   // x16 - Temporary register
    A3,   // x17 - Temporary register
    A4,   // x18 - Saved register
    A5,   // x19 - Saved register
    A6,   // x20 - Saved register
    A7,   // x21 - Saved register
    S2,   // x22 - Saved register
    S3,   // x23 - Saved register
    S4,   // x24 - Saved register
    S5,   // x25 - Saved register
    S6,   // x26 - Saved register
    S7,   // x27 - Saved register
    S8,   // x28 - Saved register
    S9,   // x29 - Saved register
    S10,  // x30 - Temporary register
    S11,  // x31 - Integer register
}

/// Represents the type of a gate.
///
/// This enum defines the possible types of gates,
/// specifically addition and multiplication gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instructions {
    Add,
    Addi,
    Sub,
    Mul,
    Div,
    Ld,
}

/// Represents a gate with its parameters.
///
/// # Fields
/// - `inx_left`: The index of the left input of the gate.
/// - `inx_right`: The index of the right input of the gate.
/// - `val_left`: Optional value for the left input, if provided.
/// - `val_right`: Optional value for the right input, if provided.
/// - `gate_type`: The type of the gate, which can be either an addition or multiplication gate.
///
/// # Description
/// This struct is used to define a gate. It includes the indices for the
/// left and right inputs, optional values for these inputs, and the type of gate being used.
#[derive(Debug)]
pub struct Gate {
    pub val_left: Option<u64>,
    pub val_right: Option<u64>,
    pub reg_left: u8,
    pub reg_right: u8,
    pub gate_type: Instructions,
}

impl Gate {
    /// Creates a new instance of a `Gate`.
    ///
    /// # Parameters
    /// - `l`: The index of the left input.
    /// - `r`: The index of the right input.
    /// - `val_left`: Optional value for the left input.
    /// - `val_right`: Optional value for the right input.
    /// - `gtype`: The type of gate (addition or multiplication).
    ///
    /// # Returns
    /// Returns a `Gate` instance with the specified parameters.
    ///
    /// # Description
    /// This constructor method initializes a `Gate` with the provided indices, optional
    /// values, and gate type.
    pub fn new(
        val_left: Option<u64>,
        val_right: Option<u64>,
        reg_left: u8,
        reg_right: u8,
        gate_type: Instructions,
    ) -> Self {
        Self {
            val_left,
            val_right,
            reg_left,
            reg_right,
            gate_type,
        }
    }
}

/// Parses a line of text into a tuple containing a specific element and a vector of elements.
///
/// # Parameters
/// - `line`: A string slice representing the line of text to be parsed.
/// - `index`: The line index used for error reporting.
///
/// # Returns
/// - `Ok`: Returns a tuple where the first element is a string slice (`&str`) corresponding to the third part of the line, and the second element is a vector of string slices (`Vec<&str>`) containing the remaining parts from the fourth onward.
/// - `Err`: Returns an error if the line does not contain at least four parts, with an error message including the line index.
///
/// # Errors
/// Returns an error if the line does not contain at least four non-empty parts, as determined by splitting on commas and spaces and filtering out empty parts.
fn parse_line(line: &str, index: usize) -> Result<(&str, Vec<&str>)> {
    // Split the input line into parts by trimming whitespace and splitting on commas and spaces
    let parts: Vec<&str> = line
        .trim()
        .split(&[',', ' '])
        .filter(|s| !s.trim().is_empty())
        .collect();
    if parts.len() >= 4 {
        // Part 0 is the instruction, and the rest are registers and numbers
        Ok((parts[0], parts[1..].to_vec()))
    } else {
        Err(anyhow!("a problem occurred in line {}", index))
    }
}

fn match_reg(reg: &str) -> Option<u8> {
    // TODO: Add zero?
    let res = match reg.to_lowercase().as_str() {
        "ra" => 1,   // x1 - Return address
        "sp" => 2,   // x2 - Stack pointer
        "gp" => 3,   // x3 - Global pointer
        "tp" => 4,   // x4 - Thread pointer
        "t0" => 5,   // x5 - Temporary register
        "t1" => 6,   // x6 - Temporary register
        "t2" => 7,   // x7 - Temporary register
        "s0" => 8,   // x8 - Platform register
        "s1" => 9,   // x9 - Platform register
        "a0" => 10,  // x10 - Argument register
        "a1" => 11,  // x11 - Argument register
        "a2" => 12,  // x12 - Temporary register
        "a3" => 13,  // x13 - Temporary register
        "a4" => 14,  // x14 - Temporary register
        "a5" => 15,  // x15 - Temporary register
        "a6" => 16,  // x16 - Temporary register
        "a7" => 17,  // x17 - Temporary register
        "s2" => 18,  // x18 - Saved register
        "s3" => 19,  // x19 - Saved register
        "s4" => 20,  // x20 - Saved register
        "s5" => 21,  // x21 - Saved register
        "s6" => 22,  // x22 - Saved register
        "s7" => 23,  // x23 - Saved register
        "s8" => 24,  // x24 - Saved register
        "s9" => 25,  // x25 - Saved register
        "s10" => 26, // x26 - Saved register
        "s11" => 27, // x27 - Saved register
        "t3" => 28,  // x28 - Temporary register
        "t4" => 29,  // x29 - Frame pointer
        "t5" => 30,  // x30 - Return address
        "t6" => 31,  // x31 - Integer register
        _ => return None,
    };
    Some(res)
}

fn register_parser(reg: Vec<&str>) -> (u8, u8) {
    let left_reg = match_reg(reg[0]).expect(format!("Invalid left register: {}", reg[0]).as_str());
    println_dbg!("reg --> {:?}, {:?}", reg[1], reg[2]);
    let right_reg = match_reg(reg[1]).unwrap_or_else( ||
        match_reg(reg[2]).expect(format!("Invalid right register: {}", reg[2]).as_str())
    );
    (left_reg, right_reg)
}

#[derive(Debug)]
pub struct RegData {
    pub init_val: Mfp,
    pub witness: Vec<(usize, Mfp)>,
}
impl RegData {
    pub fn new(val: Mfp) -> Self {
        Self {
            init_val: val,
            witness: vec![],
        }
    }
}


pub fn parse_from_lines(line_file: Vec<usize>, opcodes_file: &PathBuf) -> Result<Vec<Gate>> {
    let mut gates = Vec::new();
    
    for line_num in line_file {
        let gates_file = open_file(opcodes_file).context("Failed to open opcodes file")?;
        let line = gates_file.lines().nth(line_num - 1).ok_or_else(|| {
            anyhow!("Line number {} is out of bounds in opcodes file", line_num)
        })??;

        let (operation, operands) = parse_line(&line, line_num)
            .context(format!("Error parsing line {}: {}", line_num, line))?;

        let gate_type = gate_type(operation);
        if let Err(ref e) = gate_type {
            // Return Err
            eprintln!("Error determining gate type for line {}: {}", line_num, e);
            continue;
        }
        let gate_type = gate_type.unwrap();

        let constant_right = operands
            .get(2)
            .ok_or_else(|| anyhow!("Missing operand at index 2 for line {}", line_num))?
            .parse::<u64>()
            .ok();
        let constant_left = operands
            .get(1)
            .ok_or_else(|| anyhow!("Missing operand at index 1 for line {}", line_num))?
            .parse::<u64>()
            .ok();

        let reg_data = register_parser(operands.clone());

        let gate = Gate::new(
            constant_left,
            constant_right,
            reg_data.0,
            reg_data.1,
            gate_type,
        );

        println_dbg!("gate ==> {:?}", gate);

        gates.push(gate);
    }

    println!("Gates:");
    println!("{:#?}", gates);
    
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
fn gate_type(op: &str) -> Result<Instructions> {
    match op {
        "addi" => Ok(Instructions::Addi),
        "add" => Ok(Instructions::Add),
        // "sub" => Ok(GateType::Sub),
        "mul" => Ok(Instructions::Mul),
        // "div" => Ok(GateType::Div),
        _ => Err(anyhow!("operation is not support: {}", op)),
    }
}

#[cfg(test)]
mod parser_test {
    use super::*;

    #[test]
    fn parse_line_func() {
        let line1 = "40380552:       02f407b3                mul     a1,s0,5";
        let line2 = "40380552:       02f407b3                add     a1, s0, 5";
        let line3 = "40380552:       02f407b3                mul     a1  ,  s0  ,  5  ";
        let line4 = "40380552:       02f407b3                ld      a1  ,  a1  ,  4  ";

        let parse1 = parse_line(line1, 1).unwrap();
        let parse2 = parse_line(line2, 2).unwrap();
        let parse3 = parse_line(line3, 3).unwrap();
        let parse4 = parse_line(line4, 4).unwrap();

        assert_eq!(parse1, ("mul", ["a1", "s0", "5"].to_vec()));
        assert_eq!(parse2, ("add", ["a1", "s0", "5"].to_vec()));
        assert_eq!(parse3, ("mul", ["a1", "s0", "5"].to_vec()));
        assert_eq!(parse4, ("ld", ["a1", "a1", "4"].to_vec()));
    }
}
