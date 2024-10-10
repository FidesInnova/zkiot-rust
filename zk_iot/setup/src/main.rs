use anyhow::{Result, Context};
use zk_iot::ahp::setup::Setup;

fn main() -> Result<()> {
    let mut setup = Setup::new();
    
    // Generate cryptographic keys for the setup
    setup.key_generate(50_000);

    // Store the generated setup data in a JSON file
    setup
        .store("zkp_data/setup.json")
        .with_context(|| "Error saving setup file")?;

    println!("Setup file generated successfully");
    Ok(())
}
