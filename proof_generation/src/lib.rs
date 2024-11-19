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

#![no_main]

use std::arch::asm;
use json_file::ClassDataJson;
use json_file::DeviceConfigJson;
use json_file::ProgramParamsJson;
use utils::read_json_file;
use zk_iot::parser::Gate;
use zk_iot::parser::Instructions::*;
use zk_iot::*;

use anyhow::{Context, Result};
use zk_iot::ahp::{self, setup::Setup};

const PROGRAM_PARAMS_PATH: &str = "data/program_params.json";
const PROGRAM_COMMITMENT_PATH: &str = "data/program_commitment.json";
const SETUP_PATH: &str = "data/setup3.json";
const DEVICE_CONFIG_PATH: &str = "data/device_config.json";
const CLASS_TABLE: &str = "class.json";
const PROOF_PATH: &str = "data/proof.json";


// Exported for use in assembly
#[export_name = "proofGenerator"]
pub fn main_proof_gen() -> Result<()> {
    // Load files
    let device_config: DeviceConfigJson = read_json_file(DEVICE_CONFIG_PATH)?;
    let class_number = device_config.class;

    // Load class data from the JSON file
    let class_data =
        ClassDataJson::get_class_data(CLASS_TABLE, class_number).with_context(|| "Error loading class data")?;

    // Restore setup data from the JSON file
    let setup_json = Setup::restore(SETUP_PATH).with_context(|| "Error retrieving setup data")?;

    // Load commitment data from the commitment file
    let commitment_json = ahp::commitment_generation::Commitment::restore(PROGRAM_COMMITMENT_PATH)
        .with_context(|| "Error loading commitment data")?;


    // TODO: Implement logic to read from registers and potentially generate vector z here

    // Load matrices
    let program_params = ProgramParamsJson::restore(PROGRAM_PARAMS_PATH)?;

    // .: Proof Generation :.
    let proof_generation = ahp::proof_generation::ProofGeneration::new();
    let proof_data = proof_generation.generate_proof(
        &setup_json.get_ck(),
        class_data,
        program_params,
        commitment_json,
    );

    // Store the generated proof data in a JSON file
    proof_generation
        .store(PROOF_PATH, proof_data)
        .with_context(|| "Error storing proof data")?;
    println!("ProofGeneration file generated successfully");

    Ok(())
}