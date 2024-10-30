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


use std::env;

use ahp::setup::Setup;
use anyhow::Context;
use anyhow::Result;

use parser::*;
use parser::GateType::*;
use zk_iot::json_file::*;
use zk_iot::*;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let setup_path = &args[1];
    let program_commitment_path = &args[2];
    let program_params_path = &args[3];

    // Load files
    // Load class data from the JSON file
    let class_data =
        get_class_data("class_table.json", "test").with_context(|| "Error loading class data")?;

    // Restore setup data from the JSON file
    let setup_json =
        Setup::restore(setup_path).with_context(|| "Error retrieving setup data")?;

    // Load commitment data from the commitment file
    let commitment_json = ahp::commitment_generation::Commitment::restore(program_commitment_path)
        .with_context(|| "Error loading commitment data")?;

    // Hardcoded gates
    let gates = include!("gates.rs");

    // Load matrices 
    let matrices = matrices::Matrices::restore(program_params_path)?;

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
