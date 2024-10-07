use anyhow::Context;
use anyhow::Result;
use zk_iot::ahp::commitment;
use zk_iot::ahp::commitment::Commitment;
use zk_iot::ahp::proof_generation::ProofGeneration;
use zk_iot::json_file::get_class_data;
use zk_iot::{
    ahp::{proof_verification::Verification, setup::Setup},
    math::{Mfp, GENERATOR},
};

fn main() -> Result<()> {
    // get data class
    let class_data = get_class_data("../class_table.json", "test")
        .with_context(|| "Failed to get class data")?;

    // get ck from setup file
    let (ck, vk) =
        Setup::restore("../zkp_data/setup.json").with_context(|| "Failed to get setup data")?;
    let (polys_px, z_vec) = Commitment::restore("../zkp_data/commit.json")?;
    let proof_generation = ProofGeneration::restore("../zkp_data/proof.json").with_context(|| "Failed to get proof data")?;

    // .: Verification :.
    let verification = Verification::new(proof_generation);
    let verification_result = verification.verify((&ck, vk), class_data, &polys_px, z_vec);

    println!("Verification result {verification_result}");
    Ok(())
}
