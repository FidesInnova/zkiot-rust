// Copyright 2024 Fidesinnova, Inc.
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


use anyhow::{Result, Context};
use zk_iot::ahp::setup::Setup;

fn main() -> Result<()> {
    let mut setup = Setup::new();

    let b = 2; // Random number 
    let d_ahp = 10_000;
    
    // Generate cryptographic keys for the setup
    setup.key_generate(d_ahp);

    // Store the generated setup data in a JSON file
    setup
        .store("zkp_data/setup.json")
        .with_context(|| "Error saving setup file")?;

    println!("Setup file generated successfully");
    Ok(())
}
