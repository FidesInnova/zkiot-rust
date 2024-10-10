use ahp::setup::Setup;
use anyhow::Context;
use anyhow::Result;

use zk_iot::json_file::*;
use zk_iot::*;

fn main() -> Result<()> {
    // Load class data from the JSON file
    let class_data =
        get_class_data("class_table.json", "test").with_context(|| "Error loading class data")?;

    // Restore setup data from the specified JSON file
    let setup_json =
        Setup::restore("zkp_data/setup.json").with_context(|| "Error retrieving setup data")?;

    // Load commitment data from the commitment file
    let commitment_json = ahp::commitment::Commitment::restore("zkp_data/commit.json")
        .with_context(|| "Error loading commitment data")?;

    // .: Proof Generation :.
    let proof_generation = ahp::proof_generation::ProofGeneration::new();
    let proof_data = proof_generation.get_proof(
        &setup_json.get_commitment_key(),
        class_data,
        commitment_json,
    );

    // Store the generated proof data in a JSON file
    proof_generation
        .store("zkp_data/proof.json", proof_data)
        .with_context(|| "Error storing proof data")?;

    Ok(())
}
