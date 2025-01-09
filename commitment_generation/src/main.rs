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

use ahp::setup::Setup;
use anyhow::Context;
use anyhow::Result;
use generate_program::generate_new_program;
use std::path::PathBuf;
use utils::read_json_file;

use clap::Parser;
use parser::*;
use zk_iot::json_file::*;
use zk_iot::*;

mod generate_program;

const PROGRAM_PARAMS_PATH: &str = "data/program_params.json";
const PROGRAM_COMMITMENT_PATH: &str = "data/program_commitment.json";
const CLASS_TABLE: &str = "class.json";

// TODO: get class numebr from args
/// A program for commitment generation
#[derive(Parser, Debug)]
#[command(name = "CommitmentGenerator")]
#[command(about = "Generates commitments based on provided configuration and setup files")]
struct Args {
    /// Path to the program that contains the opcodes
    #[arg(required = true)]
    program_path: String,

    /// Path to the setup file
    #[arg(required = true)]
    setup_path: String,

    /// Path to the device configuration
    #[arg(required = true)]
    device_config_path: String,
}

fn main() -> Result<()> {
    // Parse the command-line arguments
    let args = Args::parse();

    // Use the extracted paths
    let program_path = &args.program_path;
    let device_config_path = &args.device_config_path;
    let setup_path = &args.setup_path;

    // Load class data from JSON file
    let classes_data = ClassDataJson::get_all_class_data(CLASS_TABLE)
        .with_context(|| "Error loading class table")?;

    // Used for automatically choosing a class (currently selected by the user)
    let mut lines_scope: Vec<u64> = classes_data.iter().map(|v| v.1.n_g).collect();
    lines_scope.sort();

    let device_config: DeviceConfigJson = read_json_file(device_config_path)?;

    // Restore setup data from the specified JSON file
    let setup_json = Setup::restore(setup_path).with_context(|| "Error retrieving setup data")?;

    // Convert line ranges to individual line numbers.
    let lines = DeviceConfigJson::convert_lines(device_config.code_block);

    // Parse opcodes based on the specified line numbers
    let gates = parse_from_lines(lines, &PathBuf::from(program_path))
        .with_context(|| "Error parsing instructions")?;


    // Get the class number based on the length of the gates
    let class_number = &device_config.class;
    // let class_number = &get_class_number(gates.len());
    println_dbg!("class: {}", class_number);

    // The number of times the instruction 'addi s1, 0' is added to the assembly code.
    let addi_instruction_count = get_addi_number(device_config.code_block, classes_data[class_number].n_g);

    let gates = ahp::commitment_generation::Commitment::process_gates(gates, classes_data[class_number].n_g);

    
    // Ensure that the P in use is correct
    assert_eq!(
        classes_data[class_number].p,
        math::P,
        "Assertion failed: Expected P to be {}, but found {} for class number {}",
        classes_data[class_number].p,
        math::P,
        class_number
    );

    // Generate new assembly file at program_commitment_path/program_new.s
    generate_new_program(
        program_path,
        device_config.code_block,
        classes_data[class_number],
        addi_instruction_count
    )?;

    // .: Commitment :.
    let commitment = ahp::commitment_generation::Commitment::new(classes_data[class_number])
        .gen_matrices(gates, classes_data[class_number].n_i.try_into()?)
        .gen_polynomials()
        .build();

    let commitment_polys = commitment.get_polynomials_commitment(&setup_json.get_ck());

    let _ = ProgramParamsJson::new(
        &commitment.matrices,
        &commitment.points_px,
        classes_data[class_number],
    )
    .store(PROGRAM_PARAMS_PATH)?;

    // Store the commitment data in a JSON file
    commitment
        .store(
            PROGRAM_COMMITMENT_PATH,
            *class_number,
            classes_data[class_number],
            device_config
        )
        .with_context(|| "Error storing commitment data")?;

    println!("Commitment file generated successfully");

    Ok(())
}


fn get_addi_number(line_range: LineValue, n_g: u64) -> u64 {
    let LineValue::Range(range) = line_range;
    let diff = (range.1 - range.0) as u64;
    let addi_instruction_count = n_g - diff - 1;
    addi_instruction_count
}


fn get_class_number(len: usize) -> u8 {
    if len == 1 {
        return 1;
    }

    let mut number = len;
    while !number.is_power_of_two() {
        number += 1;
    }
    (number as f64).log2() as u8
}
