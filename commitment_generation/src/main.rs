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
use std::path::PathBuf;

use parser::*;
use zk_iot::json_file::*;
use zk_iot::math::*;
use zk_iot::*;
fn main() -> Result<()> {
    // Load class data from JSON file
    let class_data =
        get_class_data("class_table.json", "test").with_context(|| "Error loading class table")?;

    // Restore setup data from the specified JSON file
    let setup_json =
        Setup::restore("zkp_data/setup.json").with_context(|| "Error retrieving setup data")?;


    // Open the file containing line numbers for opcode reading
    let line_file = open_file(&PathBuf::from("line_num.txt"))
        .with_context(|| "Error opening line number file")?;

    // Parse opcodes based on the specified line numbers
    let gates = parse_from_lines(line_file, &PathBuf::from("sample.txt"))
        .with_context(|| "Error parsing instructions")?;

    // .: Commitment :.
    let commitment = ahp::commitment_generation::Commitment::new(class_data)
        .gen_matrices(gates, class_data.n_i.try_into()?)
        .gen_polynomials()
        .build();


    // Generate polynomial commitments
    let commitment_polys =
        commitment.get_polynomials_commitment(&setup_json.get_commitment_key());

    // Store the matrices data in a JSON file
    commitment.matrices.store("zkp_data/program_params.json", &class_data)?;

    // Store the commitment data in a JSON file
    commitment
        .store("zkp_data/program_commitment.json")
        .with_context(|| "Error storing commitment data")?;
    println!("Commitment file generated successfully");

    Ok(())
}
