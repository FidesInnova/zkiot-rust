use std::path::PathBuf;

use ahp::setup::Setup;
use anyhow::Context;
use anyhow::Result;

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

    // Open the file containing line numbers for opcode reading
    let line_file = open_file(&PathBuf::from("line_num.txt"))
        .with_context(|| "Error opening line number file")?;

    // Parse opcodes based on the specified line numbers
    let gates = parse_from_lines(line_file, &PathBuf::from("sample.txt"))
        .with_context(|| "Error parsing instructions")?;

    // Load commitment data from the commitment file
    let commitment_json = ahp::commitment::Commitment::restore("zkp_data/commit.json")
        .with_context(|| "Error loading commitment data")?;

    // Generate the associated matrices
    let commitment = ahp::commitment::Commitment::new(class_data)
        .gen_matrices(gates, class_data.n_i.try_into()?)
        .build();

    // .: Proof Generation :.
    let proof_generation = ahp::proof_generation::ProofGeneration::new();
    let proof_data = proof_generation.get_proof(
        &setup_json.get_commitment_key(),
        class_data,
        commitment_json,
        commitment
    );

    // Store the generated proof data in a JSON file
    proof_generation
        .store("zkp_data/proof.json", proof_data)
        .with_context(|| "Error storing proof data")?;
    println!("ProofGeneration file generated successfully");
    Ok(())
}
