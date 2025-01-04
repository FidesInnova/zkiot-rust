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
use zk_iot::{ahp::setup::Setup, json_file::ClassDataJson, println_dbg};


const CLASS_TABLE: &str = "class.json";

fn main() -> Result<()> {
    let mut setup = Setup::default();
    
    // Load class data from the JSON file
    let class_data =
        ClassDataJson::get_all_class_data(CLASS_TABLE).with_context(|| "Error loading class data")?;

    // Create a setup file for each entry in class_data
    for (class_number, metadata) in class_data {
        // Calculate the D_AHP value using the formula: D_AHP = 12 * n_g

        let d_ahp_vec: Vec<u64> = vec![3 * metadata.n_g + 2 * metadata.n_i + 2, 12 * metadata.n_g];
        let d_ahp = *d_ahp_vec.iter().max().unwrap();

        let inx = d_ahp_vec.iter().position(|v| *v == d_ahp).unwrap();
        println_dbg!("class_number {class_number}: inx {}, number: {}", inx, d_ahp_vec[inx]);

        // Generate cryptographic keys for the setup
        setup.generate_keys(d_ahp, metadata.p, metadata.g);

        // Save the generated setup data to a JSON file
        setup
            .store(&format!("data/setup{}.json", class_number), class_number)
            .with_context(|| "Error saving setup file")?;
    }
    
    println!("Setup file generated successfully");
    Ok(())
}
