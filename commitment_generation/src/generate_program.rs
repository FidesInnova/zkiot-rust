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
use std::collections::HashMap;
use std::f32::RADIX;
use std::io::{BufReader, Read, Write};
use std::{
    fs::{File, OpenOptions},
    io::BufRead,
    path::{Path, PathBuf},
};
use zk_iot::json_file::{ClassDataJson, LineValue};
use zk_iot::parser::{match_reg, parse_from_lines, parse_line};

pub fn generate_new_program(
    input_path: &str,
    line_range: LineValue,
    class_data: ClassDataJson,
) -> Result<()> {
    // Open the input file
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    let n_g = class_data.n_g;

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
        n_g.try_into()?,
        add_no_op_number,
    )?;

    Ok(())
}

fn insert_assembly_instructions(
    output_file: &mut File,
    reader: BufReader<File>,
    line_range: (usize, usize),
    n_g: usize,
    add_no_op_number: u64,
) -> Result<()> {
    // Allocating memory for the generated ASM file!
    let mut space_size = vec![4; 32];

    for (num, line) in reader.lines().enumerate() {
        let num = num + 1;
        let instruction = line?;


        if num == line_range.0 {
            writeln!(output_file, "    jal store_register_instances")?;
        }

        writeln!(output_file, "{}", instruction)?;

        if num >= line_range.0 && num <= line_range.1 {
            // Parsing the destination register from the instruction
            let des_reg = parse_line(&instruction, num)?.1[0];
            let des_reg_num = match_reg(des_reg).ok_or_else(|| anyhow!("Match register faild"))? as usize;
            writeln!(output_file, "    sw x{}, x{}_array({})", des_reg_num, des_reg_num, space_size[des_reg_num])?;
            space_size[des_reg_num] += 4;
        }

        if num == line_range.1 {
            // for _ in 0..add_no_op_number {
            //     writeln!(output_file, "    nop")?;
            // }
            writeln!(output_file, "    jal proofGenerator")?;
        }
    }


    insert_arrays(output_file, space_size)?;

    insert_store_register_function(output_file, n_g)?;

    Ok(())
}

fn insert_arrays(output_file: &mut File, space_size: Vec<u64>) -> Result<()> {
    writeln!(output_file, "    .data")?;
    for (num, size) in space_size.iter().enumerate() {
        writeln!(output_file, "x{}_array:    .space {}   # Array for x{}", num, size, num)?;
    }
    Ok(())
}

fn insert_store_register_function(output_file: &mut File, n_g: usize) -> Result<()> {
    // Save register function
    writeln!(
        output_file,
        r#"{}"#,
        include_str!("../store_registers.asm").replace("SPACE_SIZE", &n_g.to_string())
    )?;
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
