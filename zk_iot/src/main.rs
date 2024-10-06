use std::path::PathBuf;

use ahp::{proof_verification::Verification, setup::Setup};
// use rand::{thread_rng, Rng};
use rustnomial::{Evaluable, FreeSizePolynomial, Polynomial};
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


    // Open line number file containing indices for reading opcodes
    let line_file = open_file(&PathBuf::from("line_num.txt"))?;
    // Reading the opcodes whose line numbers are specified in line_file
    let gates = parse_from_lines(line_file, &PathBuf::from("sample.txt"))?;


    // .: Commitment :.
    let commitmnet = ahp::commitment::Commitment::new(&setup)
                        .gen_matrices(gates, setup.number_input.try_into()?)
                        .gen_polynomials()
                        .build();

    let commitmnet_polys_ = commitmnet.get_polynomials_commitment(setup.generator, &commitment_key);

    
    // .: Proof Generation :.
    let proof_generation = ahp::proof_generation::ProofGeneration::new()
                        .get_proof(&commitmnet, &commitment_key, setup.generator);

    
    // .: Verification :.
    let verification = Verification::new(proof_generation);
    let verification_result = verification.verify(&commitmnet, verifying_key);

    println!("Verification result {verification_result}");
    Ok(())
}