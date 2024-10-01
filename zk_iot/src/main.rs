use std::path::PathBuf;

// use rand::{thread_rng, Rng};
use rustnomial::{Evaluable, FreeSizePolynomial, Polynomial};
use setup::Setup;
use anyhow::Result;
use ark_ff::Field;

use parser::parse_from_lines;
use zk_iot::*;
use utils::*;
use math::*;
use json_file::*;


fn main() -> Result<()> {
    // .:‌Setup :.
    let setup = Setup::new();
    let (commitment_key, verifying_key) = setup.key_generate();

    // .: Commitment :.
    // Open line number file containing indices for reading opcodes
    let line_file = open_file(&PathBuf::from("line_num.txt"))?;
    // Reading the opcodes whose line numbers are specified in line_file
    let gates = parse_from_lines(line_file, &PathBuf::from("sample.txt"))?;

    let commitmnet = ahp::commitmnet::Commitment::new(&setup)
                        .build_matrices(gates, setup.number_input.try_into()?)
                        .build_polynomials()
                        .build();

    let commitmnet_polys_ = commitmnet.get_polynomials_commitment(setup.generator, &commitment_key);

    
    let proof_generation = ahp::proof_generation::ProofGeneration::new()
                                .get_proof(commitmnet, &commitment_key, setup.generator);
                            
    
    // ProofGeneration
    // Verification
    Ok(())
}
