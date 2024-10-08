use ahp::setup::Setup;
use anyhow::Context;
use anyhow::Result;
use std::path::PathBuf;

use parser::*;
use zk_iot::json_file::*;
use zk_iot::math::*;
use zk_iot::*;

fn main() -> Result<()> {
    // Get data class
    let class_data = get_class_data("class_table.json", "test")
        .with_context(|| "Failed to get class data")?;

    // Get commitment key from setup file
    let (ck, _) =
        Setup::restore("zkp_data/setup.json").with_context(|| "Failed to get setup data")?;

    // Open line number file containing indices for reading opcodes
    let line_file =
        open_file(&PathBuf::from("line_num.txt")).with_context(|| "Failed to get line_num")?;

    // Reading the opcodes whose line numbers are specified in line_file
    let gates = parse_from_lines(line_file, &PathBuf::from("sample.txt"))
        .with_context(|| "Failed to get instructions")?;

    
    // .: Commitment :.
    let commitmnet = ahp::commitment::Commitment::new(class_data)
        .gen_matrices(gates, class_data.n_i.try_into()?)
        .gen_polynomials()
        .build();
    commitmnet.store("zkp_data/commit.json").with_context(|| "Failed to store commitment")?;


    // .: Proof Generation :.
    let proof_generation = ahp::proof_generation::ProofGeneration::new();
    let proof_data = proof_generation.get_proof(&commitmnet, &ck);
    proof_generation.store("zkp_data/proof.json", proof_data).with_context(|| "Failed to store proof data")?;

    Ok(())
}