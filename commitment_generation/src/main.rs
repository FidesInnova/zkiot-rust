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
use std::env;
use std::path::PathBuf;
use utils::read_json_file;

use parser::*;
use zk_iot::json_file::*;
use zk_iot::*;
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let program_path = &args[1];
    let device_config_path = &args[2];
    let setup_path = &args[3];

    // Load class data from JSON file
    let class_data =
        get_class_data("class_table.json", "test").with_context(|| "Error loading class table")?;

    let device_config: DeviceConfigJson = read_json_file(device_config_path)?;

    // Restore setup data from the specified JSON file
    let setup_json = Setup::restore(setup_path).with_context(|| "Error retrieving setup data")?;
    
    // Convert line ranges to individual line numbers.
    let lines = convert_lines(device_config.lines);

    // Parse opcodes based on the specified line numbers
    let gates = parse_from_lines(lines, &PathBuf::from(program_path))
        .with_context(|| "Error parsing instructions")?;

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
        .store("data/program_params.json", &class_data)?;

    // Store the commitment data in a JSON file
    commitment
        .store("data/program_commitment.json")
        .with_context(|| "Error storing commitment data")?;
    println!("Commitment file generated successfully");

    Ok(())
}