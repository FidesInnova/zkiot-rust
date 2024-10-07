use anyhow::Context;
use zk_iot::{ahp::setup::Setup, json_file::{get_class_data, ClassData}};

fn main() -> anyhow::Result<()> {
    let mut setup = Setup::new();
    setup.key_generate();

    setup
        .store("../zkp_data/setup.json")
        .with_context(|| "Failed to generate setup file")?;

    println!("Setup file generated successfully");
    Ok(())
}
