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
    let commitment = ahp::commitment::Commitment::new(class_data)
        .gen_matrices(gates, class_data.n_i.try_into()?)
        .gen_polynomials()
        .build();

    // Generate polynomial commitments
    let commitment_polys =
        commitment.get_polynomials_commitment(GENERATOR, &setup_json.get_commitment_key());

    // Store the commitment data in a JSON file
    commitment
        .store("zkp_data/commit.json")
        .with_context(|| "Error storing commitment data")?;
    Ok(())
}
