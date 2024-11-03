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
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use utils::read_json_file;

use parser::*;
use zk_iot::json_file::*;
use zk_iot::*;

use clap::Parser;

/// A program for commitment generation
#[derive(Parser, Debug)]
#[command(name = "CommitmentGenerator")]
#[command(about = "Generates commitments based on provided configuration and setup files")]
struct Args {
    /// Path to the program
    #[arg(required = true)]
    program_commitment_path: String,

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
    let program_commitment_path = &args.program_commitment_path;
    let device_config_path = &args.device_config_path;
    let setup_path = &args.setup_path;
    
    // Load class data from JSON file
    let class_data =
        get_class_data("class_table.json", "test").with_context(|| "Error loading class table")?;

    let device_config: DeviceConfigJson = read_json_file(device_config_path)?;

    // Restore setup data from the specified JSON file
    let setup_json = Setup::restore(setup_path).with_context(|| "Error retrieving setup data")?;

    // Convert line ranges to individual line numbers.
    let lines = convert_lines(device_config.lines);

    // Parse opcodes based on the specified line numbers
    let gates = parse_from_lines(lines, &PathBuf::from(program_commitment_path))
        .with_context(|| "Error parsing instructions")?;

    generate_new_program(program_commitment_path);

    // .: Commitment :.
    let commitment = ahp::commitment_generation::Commitment::new(class_data)
        .gen_matrices(gates, class_data.n_i.try_into()?)
        .gen_polynomials()
        .build();

    // Generate polynomial commitments
    let commitment_polys = commitment.get_polynomials_commitment(&setup_json.get_ck());

    // Store the matrices data in a JSON file
    commitment
        .matrices
        .store("data/program_params.json", setup_json, &class_data)?;

    // Store the commitment data in a JSON file
    commitment
        .store("data/program_commitment.json")
        .with_context(|| "Error storing commitment data")?;
    println!("Commitment file generated successfully");

    Ok(())
}

fn generate_new_program(input_path: &str) -> Result<()> {
    // Open the input file
    let input_file = File::open(input_path)?;
    let reader = std::io::BufReader::new(input_file);

    let output_path = create_output_path(input_path);
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)?;

    
    // Write the assembly function
    writeln!(
        output_file,
        r#"
.section .text
.global save_registers
save_registers:
    la t0, registers     # Load the starting address of storage space
    # Storing register values
    sw zero, 0(t0)       # Store the value of register zero
    sw ra, 4(t0)         # Store the value of register ra
    sw sp, 8(t0)         # Store the value of register sp
    sw gp, 12(t0)        # Store the value of register gp
    sw tp, 16(t0)        # Store the value of register tp
    sw t0, 20(t0)        # Store the value of register t0
    sw t1, 24(t0)        # Store the value of register t1
    sw t2, 28(t0)        # Store the value of register t2
    sw s0, 32(t0)        # Store the value of register s0
    sw s1, 36(t0)        # Store the value of register s1
    sw s2, 40(t0)        # Store the value of register s2
    sw s3, 44(t0)        # Store the value of register s3
    sw s4, 48(t0)        # Store the value of register s4
    sw s5, 52(t0)        # Store the value of register s5
    sw s6, 56(t0)        # Store the value of register s6
    sw s7, 60(t0)        # Store the value of register s7
    sw s8, 64(t0)        # Store the value of register s8
    sw s9, 68(t0)        # Store the value of register s9
    sw s10, 72(t0)       # Store the value of register s10
    sw s11, 76(t0)       # Store the value of register s11
    sw a0, 80(t0)        # Store the value of register a0
    sw a1, 84(t0)        # Store the value of register a1
    sw a2, 88(t0)        # Store the value of register a2
    sw a3, 92(t0)        # Store the value of register a3
    sw a4, 96(t0)        # Store the value of register a4
    sw a5, 100(t0)       # Store the value of register a5
    sw a6, 104(t0)       # Store the value of register a6
    sw a7, 108(t0)       # Store the value of register a7
    ret                  # Return from the function
"#
    )?;


    writeln!(output_file, "    jal save_registers")?; // Call before the instruction
    for line in reader.lines() {
        let instruction = line?;
        writeln!(output_file, "    {}", instruction)?; 
        writeln!(output_file, "    jal save_registers")?;
    }

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
