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

use anyhow::{anyhow, Result};
use std::io::{BufReader, Write};
use std::{
    fs::{File, OpenOptions},
    io::BufRead,
    path::{Path, PathBuf},
};
use zk_iot::json_file::{ClassDataJson, LineValue};
use zk_iot::parser::{match_reg, parse_line};

pub fn generate_new_program(
    input_path: &str,
    line_range: LineValue,
    class_data: ClassDataJson,
) -> Result<()> {
    // Open the input file
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    let n_g = class_data.n_g;
    let n_i = class_data.n_i;

    // Create output file path
    let output_path = create_output_path(input_path);
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)?;

    let LineValue::Range(range) = line_range;

    let diff = (range.1 - range.0) as u64;
    let add_no_op_number = n_g - diff - 1;

    insert_assembly_instructions(
        &mut output_file,
        reader,
        range,
        (n_g + n_i + 1).try_into()?,
        add_no_op_number,
    )?;

    Ok(())
}

fn insert_assembly_instructions(
    output_file: &mut File,
    reader: BufReader<File>,
    line_range: (usize, usize),
    z_vec_len: usize,
    add_no_op_number: u64,
) -> Result<()> {
    // Allocating memory for the generated ASM file!
    let mut space_size = vec![4; 32];

    let mut array_offset_pair: Vec<(usize, usize)> = vec![];

    for (num, line) in reader.lines().enumerate() {
        let num = num + 1;
        let instruction = line?;

        if num == line_range.0 {
            writeln!(output_file, "jal store_register_instances")?;
        }

        writeln!(output_file, "{}", instruction)?;

        if num >= line_range.0 && num <= line_range.1 {
            // Parsing the destination register from the instruction
            let parse_line = parse_line(&instruction, num)?;

            let registers = parse_line.1;
            let des = registers[0];
            let des_reg_num =
                match_reg(des).ok_or_else(|| anyhow!("Match register faild"))? as usize;
            array_offset_pair.push((des_reg_num, space_size[des_reg_num]));
            
            let x_reg = &format!("x{}", des_reg_num);
            writeln!(output_file, "la t0, {x_reg}_array")?;
            writeln!(output_file, "sw {x_reg}, {}(t0)", space_size[des_reg_num])?;

            space_size[des_reg_num] += 4;
        }

        if num == line_range.1 {
            // insert_addi_0(output_file, add_no_op_number)?;
            insert_z_array(output_file)?;
            insert_z_array_population_code(output_file)?;

            for i in 33..(z_vec_len) {
                writeln!(output_file, "la a1, x{}_array", array_offset_pair[i - 33].0)?;
                writeln!(output_file, "lw t0, {}(a1)", array_offset_pair[i - 33].1)?;
                writeln!(output_file, "sw t0, {}(a0)", i * 4)?;
            }

            writeln!(output_file, "call proofGenerator")?;
        }
    }

    insert_z_array_definition(output_file, z_vec_len)?;
    insert_arrays(output_file, space_size)?;
    insert_store_register_function(output_file)?;
    
    Ok(())
}

fn insert_addi_0(output_file: &mut File, add_no_op_number: u64) -> Result<()> {
    for _ in 0..add_no_op_number {
        writeln!(output_file, "addi s1, s1, 0")?;
    }
    Ok(())
}

fn insert_z_array_population_code(output_file: &mut File) -> Result<()> {
    for i in 1..=32 {
        writeln!(output_file, "la a0, z_array")?;
        writeln!(output_file, "la a1, x{}_array", i - 1)?;
        writeln!(output_file, "lw t0, 0(a1)")?;
        writeln!(output_file, "sw t0, {}(a0)", i * 4)?;
    }
    Ok(())
}
 
fn insert_z_array(output_file: &mut File) -> Result<()> {
    writeln!(output_file, "la a0, z_array")?;
    writeln!(output_file, "li t0, 1")?;
    writeln!(output_file, "sw t0, 0(a0)")?;
    Ok(())
}

fn insert_z_array_definition(output_file: &mut File, z_vec_len: usize) -> Result<()> {
    writeln!(output_file, ".section .data")?;
    writeln!(output_file, ".global z_array")?;
    writeln!(output_file, "z_array:    .space {}", z_vec_len * 4)?;
    Ok(())
}

fn insert_arrays(output_file: &mut File, space_size: Vec<usize>) -> Result<()> {
    writeln!(output_file, "    .data")?;
    for (num, size) in space_size.iter().enumerate() {
        writeln!(
            output_file,
            "x{}_array:    .space {}   # Array for x{}",
            num, size, num
        )?;
    }
    Ok(())
}

fn insert_store_register_function(output_file: &mut File) -> Result<()> {
    // Save register function
    writeln!(output_file, r#"{}"#, include_str!("../store_registers.asm"))?;
    Ok(())
}

fn create_output_path(input: &str) -> PathBuf {
    let path = Path::new(input);
    let parent = path.parent().unwrap();
    let file_stem = path.file_stem().unwrap();
    let new_file_name = format!(
        "{}_new{}",
        file_stem.to_string_lossy(),
        path.extension()
            .map_or(String::new(), |ext| format!(".{}", ext.to_string_lossy()))
    );
    parent.join(new_file_name)
}
