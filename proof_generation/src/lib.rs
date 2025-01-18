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

use json_file::ClassDataJson;
use json_file::DeviceConfigJson;
use json_file::ProgramParamsJson;
use utils::read_json_file;
use zk_iot::*;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{Context, Result};
use zk_iot::ahp::{self, setup::Setup};

const PROGRAM_PARAMS_PATH: &str = "data/program_params.json";
const PROGRAM_COMMITMENT_PATH: &str = "data/program_commitment.json";
const DEVICE_CONFIG_PATH: &str = "data/device_config.json";
const CLASS_TABLE: &str = "class.json";
const PROOF_PATH: &str = "data/proof.json";


// Exported for use in assembly
#[export_name = "proofGenerator"]
pub fn main_proof_gen(setup_path: &str) -> Result<()> {
    // Load commitment data from the commitment file
    let commitment_json = ahp::commitment_generation::Commitment::restore(PROGRAM_COMMITMENT_PATH)
        .with_context(|| "Error loading commitment data")?;
    let class_number = commitment_json.info.class;

    // Load class data from the JSON file
    let class_data =
        ClassDataJson::get_class_data(CLASS_TABLE, class_number).with_context(|| "Error loading class data")?;

    // Restore setup data from the JSON file
    let setup_json = Setup::restore(setup_path).with_context(|| "Error retrieving setup data")?;

    // Load matrices
    let program_params = ProgramParamsJson::restore(PROGRAM_PARAMS_PATH)?;

    let z_vec: Vec<u64> = read_vector_from_file();

    // .: Proof Generation :.
    let proof_generation = ahp::proof_generation::ProofGeneration::new();
    // Set timer 
    let timer = std::time::Instant::now();
    let proof_data = proof_generation.generate_proof(
        &setup_json.get_ck(),
        class_data,
        program_params,
        commitment_json.clone(),
        z_vec,
        class_data.p
    );
    println!("Proof timer: {:.2} milliseconds", timer.elapsed().as_millis() as f64);

    // Store the generated proof data in a JSON file
    proof_generation
        .store(PROOF_PATH, proof_data, class_number, commitment_json.info.commitment_id)
        .with_context(|| "Error storing proof data")?;
    println!("ProofGeneration file generated successfully");

    Ok(())
}


fn read_vector_from_file() -> Vec<u64> {
    let path = "proof_generation/z_vec.txt";
    let mut values = vec![];
    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                for value in line.split(',') {
                    if let Ok(num) = value.trim().parse::<u64>() {
                        values.push(num);
                    }
                }
            }
        }
    } else {
        panic!("Could not open the file: {}", path);
    }

    values
}