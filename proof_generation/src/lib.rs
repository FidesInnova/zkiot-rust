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
use zk_iot::parser::Gate;
use zk_iot::parser::GateType::*;
use zk_iot::*;

use anyhow::{Context, Result};
use zk_iot::{
    ahp::{self, setup::Setup},
    json_file::get_class_data,
    matrices,
};

const PROGRAM_PARAMS_PATH: &str = "data/program_params.json";
const PROGRAM_COMMITMENT_PATH: &str = "data/program_commitment.json";
const SETUP_PATH: &str = "data/setuptest.json";

// Exported for use in assembly
#[export_name = "proofGen"]
pub fn proof_gen() -> Result<()> {
    // Load files
    // Load class data from the JSON file
    let class_data =
        get_class_data("class_table.json", "test").with_context(|| "Error loading class data")?;

    // Restore setup data from the JSON file
    let setup_json = Setup::restore(SETUP_PATH).with_context(|| "Error retrieving setup data")?;

    // Load commitment data from the commitment file
    let commitment_json = ahp::commitment_generation::Commitment::restore(PROGRAM_COMMITMENT_PATH)
        .with_context(|| "Error loading commitment data")?;


    // TODO: Implement logic to read from registers and potentially generate vector z here

    // Temporary hardcoded gates for initial testing; will replace with dynamic reading from registers in the future
    let gates = include!("gates.rs");

    // Load matrices
    let matrices = matrices::Matrices::restore(PROGRAM_PARAMS_PATH)?;

    // .: Proof Generation :.
    let proof_generation = ahp::proof_generation::ProofGeneration::new();
    let proof_data = proof_generation.generate_proof(
        &setup_json.get_ck(),
        class_data,
        matrices,
        commitment_json,
        gates,
    );

    // Store the generated proof data in a JSON file
    proof_generation
        .store("data/proof.json", proof_data)
        .with_context(|| "Error storing proof data")?;
    println!("ProofGeneration file generated successfully");

    Ok(())
}

#[export_name = "saveReg"]
#[inline(never)]
fn read_registers() -> [u64; 32] {
    let mut registers = [0u64; 32];

    unsafe {
        // First 16 registers (x0 to x15)
        asm!(
            "mv {0}, x0",
            "mv {1}, x1",
            "mv {2}, x2",
            "mv {3}, x3",
            "mv {4}, x4",
            "mv {5}, x5",
            "mv {6}, x6",
            "mv {7}, x7",
            "mv {8}, x8",
            "mv {9}, x9",
            "mv {10}, x10",
            "mv {11}, x11",
            "mv {12}, x12",
            "mv {13}, x13",
            "mv {14}, x14",
            "mv {15}, x15",
            out(reg) registers[0],
            out(reg) registers[1],
            out(reg) registers[2],
            out(reg) registers[3],
            out(reg) registers[4],
            out(reg) registers[5],
            out(reg) registers[6],
            out(reg) registers[7],
            out(reg) registers[8],
            out(reg) registers[9],
            out(reg) registers[10],
            out(reg) registers[11],
            out(reg) registers[12],
            out(reg) registers[13],
            out(reg) registers[14],
            out(reg) registers[15],
        );

        // Second 16 registers (x16 to x31)
        asm!(
            "mv {0}, x16",
            "mv {1}, x17",
            "mv {2}, x18",
            "mv {3}, x19",
            "mv {4}, x20",
            "mv {5}, x21",
            "mv {6}, x22",
            "mv {7}, x23",
            "mv {8}, x24",
            "mv {9}, x25",
            "mv {10}, x26",
            "mv {11}, x27",
            "mv {12}, x28",
            "mv {13}, x29",
            "mv {14}, x30",
            "mv {15}, x31",
            out(reg) registers[16],
            out(reg) registers[17],
            out(reg) registers[18],
            out(reg) registers[19],
            out(reg) registers[20],
            out(reg) registers[21],
            out(reg) registers[22],
            out(reg) registers[23],
            out(reg) registers[24],
            out(reg) registers[25],
            out(reg) registers[26],
            out(reg) registers[27],
            out(reg) registers[28],
            out(reg) registers[29],
            out(reg) registers[30],
            out(reg) registers[31],
        );
    }

    registers
}