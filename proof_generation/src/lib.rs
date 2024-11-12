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


// Declare the arrays from assembly as external variables
extern "C" {
    static x0_array: *const u32;
    static x1_array: *const u32;
    static x2_array: *const u32;
    static x3_array: *const u32;
    static x4_array: *const u32;
    static x5_array: *const u32;
    static x6_array: *const u32;
    static x7_array: *const u32;
    static x8_array: *const u32;
    static x9_array: *const u32;
    static x10_array: *const u32;
    static x11_array: *const u32;
    static x12_array: *const u32;
    static x13_array: *const u32;
    static x14_array: *const u32;
    static x15_array: *const u32;
    static x16_array: *const u32;
    static x17_array: *const u32;
    static x18_array: *const u32;
    static x19_array: *const u32;
    static x20_array: *const u32;
    static x21_array: *const u32;
    static x22_array: *const u32;
    static x23_array: *const u32;
    static x24_array: *const u32;
    static x25_array: *const u32;
    static x26_array: *const u32;
    static x27_array: *const u32;
    static x28_array: *const u32;
    static x29_array: *const u32;
    static x30_array: *const u32;
    static x31_array: *const u32;
}

// fn get_array(reg_num: u8) -> Vec<u64> {
//     // Determine which array to use based on reg_num
//     let reg_vec: *const i32 = unsafe {
//         match reg_num {
//             0 => x0_array,
//             1 => x1_array,
//             2 => x2_array,
//             3 => x3_array,
//             4 => x4_array,
//             5 => x5_array,
//             6 => x6_array,
//             7 => x7_array,
//             8 => x8_array,
//             9 => x9_array,
//             10 => x10_array,
//             11 => x11_array,
//             12 => x12_array,
//             13 => x13_array,
//             14 => x14_array,
//             15 => x15_array,
//             16 => x16_array,
//             17 => x17_array,
//             18 => x18_array,
//             19 => x19_array,
//             20 => x20_array,
//             21 => x21_array,
//             22 => x22_array,
//             23 => x23_array,
//             24 => x24_array,
//             25 => x25_array,
//             26 => x26_array,
//             27 => x27_array,
//             28 => x28_array,
//             29 => x29_array,
//             30 => x30_array,
//             31 => x31_array,
//             _ => panic!("Invalid register number: {}", reg_num), // Handle out-of-bounds case
//         }
//     };

//     let mut result = Vec::new();
//     if !reg_vec.is_null() {
//         // Assuming a fixed size for the arrays, you may need to adjust this
//         let size = 32; // Change this to the actual size of your arrays
//         for i in 0..size {
//             unsafe {
//                 // Read the value from the raw pointer and convert to u64
//                 let value = *reg_vec.add(i) as u64;
//                 result.push(value);
//             }
//         }
//     }

//     result
// }


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

    // Temporary hardcoded gates for initial testing; will replace with dynamic reading from registers in the future
    let gates = include!("gates.rs");

    // Load matrices
    let program_params = ProgramParamsJson::restore(PROGRAM_PARAMS_PATH)?;

    // .: Proof Generation :.
    let proof_generation = ahp::proof_generation::ProofGeneration::new();
    let proof_data = proof_generation.generate_proof(
        &setup_json.get_ck(),
        class_data,
        program_params,
        commitment_json,
        gates,
    );

    // Store the generated proof data in a JSON file
    proof_generation
        .store(PROOF_PATH, proof_data)
        .with_context(|| "Error storing proof data")?;
    println!("ProofGeneration file generated successfully");

    Ok(())
}