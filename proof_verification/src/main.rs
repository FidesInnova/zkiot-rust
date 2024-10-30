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

use anyhow::Context;
use anyhow::Result;
use zk_iot::ahp::commitment_generation;
use zk_iot::ahp::commitment_generation::Commitment;
use zk_iot::ahp::proof_generation::ProofGeneration;
use zk_iot::json_file::get_class_data;
use zk_iot::{
    ahp::{proof_verification::Verification, setup::Setup},
    math::{Mfp, GENERATOR},
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let program_path = &args[1];
    let proof_path = &args[2];


    // Load class data from the JSON file
    let class_data = get_class_data("class_table.json", "test")
        .with_context(|| "Error loading class data")?;

    // Restore setup data from the specified JSON file
    let setup_json =
        Setup::restore("zkp_data/setup.json").with_context(|| "Error retrieving setup data")?;
        
    // Load commitment data from the commitment file
    let commitment_json = Commitment::restore(program_path)
        .with_context(|| "Error loading commitment data")?;
    
    // Load proof generation data from the proof file
    let proof_generation = ProofGeneration::restore(proof_path)
        .with_context(|| "Error loading proof data")?;

    // .: Verification :.
    let verification = Verification::new(&proof_generation);
    let verification_result = verification.verify(
        (&setup_json.get_commitment_key(), setup_json.get_verifying_key()), 
        class_data, 
        commitment_json.get_polys_px(), 
        proof_generation.get_x_vec()
    );

    eprintln!("Verification result: {}", verification_result);
    Ok(())
}
