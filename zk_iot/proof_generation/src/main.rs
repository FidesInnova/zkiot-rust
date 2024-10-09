use ahp::setup::Setup;
use anyhow::Context;
use anyhow::Result;

use zk_iot::json_file::*;
use zk_iot::*;

fn main() -> Result<()> {
    // Get data class
    let class_data = get_class_data("class_table.json", "test")
        .with_context(|| "Failed to get class data")?;

    // Load commitment key from setup file
    let (ck, _) =
        Setup::restore("zkp_data/setup.json").with_context(|| "Failed to get setup data")?;

    // Load commitment data from commit file
    let commitmnet_json = ahp::commitment::Commitment::restore("zkp_data/commit.json")?;

    // .: Proof Generation :.
    let proof_generation = ahp::proof_generation::ProofGeneration::new();
    let proof_data = proof_generation.get_proof(&ck, class_data, commitmnet_json);
    proof_generation.store("zkp_data/proof.json", proof_data).with_context(|| "Failed to store proof data")?;

    Ok(())
}