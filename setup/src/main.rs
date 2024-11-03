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
use rand::{thread_rng, Rng};
use zk_iot::{ahp::setup::Setup, json_file::{get_all_class_data, get_class_data}};

fn main() -> Result<()> {
    // 
    let mut setup = Setup::default();
    
    // Load class data from the JSON file
    let class_data =
        get_all_class_data("class_table.json").with_context(|| "Error loading class data")?;
    
    // TODO: Uncomment the following code when all tests pass 
    // Random number in range (1-Ng)
    // let b = thread_rng().gen_range(1..class_data.n_g);

    // Temporary assignment for random number b
    let b = 2; 

    // Create a setup file for each entry in class_data
    for (name, metadata) in class_data {
        // Calculate the D_AHP value using the formula: D_AHP = 12 * n_g
        let d_ahp = 12 * metadata.n_g;
        
        // Generate cryptographic keys for the setup
        setup.generate_keys(d_ahp);

        // Save the generated setup data to a JSON file
        setup
            .store(&format!("data/setup{}.json", name))
            .with_context(|| "Error saving setup file")?;
    }
    
    println!("Setup file generated successfully");
    Ok(())
}
