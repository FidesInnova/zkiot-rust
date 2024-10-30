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


use std::path::PathBuf;

use ahp::setup::Setup;
use anyhow::Context;
use anyhow::Result;

use matrices::matrix_size;
use matrices::matrix_t_zeros;
use matrices::Matrices;
use parser::parse_from_lines;
use zk_iot::json_file::*;
use zk_iot::*;

fn main() -> Result<()> {
    // Load files
    // Load class data from the JSON file
    let class_data =
        get_class_data("class_table.json", "test").with_context(|| "Error loading class data")?;

    // Restore setup data from the specified JSON file
    let setup_json =
        Setup::restore("zkp_data/setup.json").with_context(|| "Error retrieving setup data")?;

    // Load commitment data from the commitment file
    let commitment_json = ahp::commitment_generation::Commitment::restore("zkp_data/program_commitment.json")
        .with_context(|| "Error loading commitment data")?;

    // Open the file containing line numbers for opcode reading
    let line_file = open_file(&PathBuf::from("line_num.txt"))
        .with_context(|| "Error opening line number file")?;

    // Parse opcodes based on the specified line numbers
    let gates = parse_from_lines(line_file, &PathBuf::from("program.s"))
        .with_context(|| "Error parsing instructions")?;

    let matrices = matrices::Matrices::restore("zkp_data/program_params.json")?;

    // .: Proof Generation :.
    let proof_generation = ahp::proof_generation::ProofGeneration::new();
    let proof_data = proof_generation.generate_proof(
        &setup_json.get_commitment_key(),
        class_data,
        matrices,
        commitment_json,
        gates,
    );

    // Store the generated proof data in a JSON file
    proof_generation
        .store("zkp_data/proof.json", proof_data)
        .with_context(|| "Error storing proof data")?;
    println!("ProofGeneration file generated successfully");
    Ok(())
}
