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


use std::fs::File;
use anyhow::Result;
use serde::Serialize;
use std::io::BufWriter;
use serde::Deserialize;

use crate::kzg;
use crate::math::Mfp;
use crate::math::GENERATOR;
use crate::json_file::write_set;
use crate::utils::read_json_file;

/// Struct for setup data with commitment and verifying keys
pub struct Setup {
    ck: Vec<Mfp>, // Commitment keys
    vk: Mfp,      // Verifying key
}

impl Setup {
    /// Creates a new `Setup` with default keys
    pub fn default() -> Self {
        Self {
            ck: Vec::default(),
            vk: Mfp::default(),
        }
    }
    
    /// Generates commitment and verifying keys
    ///
    /// # Parameters
    /// - `num`: Number of keys to generate.
    pub fn generate_keys(&mut self, num: u64) {
        // TODO: Replace with a random number in the range
        let tau = 119;  // Placeholder for a random number

        // Generate commitment keys using KZG.
        let ck = kzg::setup(num, tau, GENERATOR);

        self.ck = ck; // Store commitment keys
        self.vk = self.ck[1]; // Set verifying key
    }

    /// Saves setup data to a JSON file
    ///
    /// # Parameters
    /// - `path`: File path to save the JSON
    pub fn store(&self, path: &str) -> Result<()> {
        let file = File::create(path)?; // Create or truncate the file
        let writer = BufWriter::new(file); // Buffer for writing

        let setup_json = SetupJson::new(&self.ck, 4); // Create JSON representation
        serde_json::to_writer(writer, &setup_json)?; // Write JSON to file
        Ok(())
    }

    /// Loads setup data from a JSON file
    ///
    /// # Parameters
    /// - `path`: File path to read the JSON
    ///
    /// # Returns
    /// Returns a `Result` with the restored `SetupJson`
    pub fn restore(path: &str) -> Result<SetupJson> {
        read_json_file(path) // Read and deserialize JSON
    }
}


/// Struct for JSON serialization and deserialization of setup data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetupJson {
    class: u8,         // Class identifier
    ck: Vec<u64>,      // Commitment keys
    vk: u64,           // Verifying key
}

impl SetupJson {
    /// Creates a new `SetupJson` from commitment keys and a class identifier
    pub fn new(ck: &Vec<Mfp>, class: u8) -> Self {
        let ck = write_set(ck); // Convert Mfp to u64
        Self {
            class,
            ck: ck.clone(), // Store commitment keys
            vk: ck[1],     // Set verifying key
        }
    }

    /// Gets commitment keys as `Mfp`.
    pub fn get_ck(&self) -> Vec<Mfp> {
        self.ck.iter().map(|v| Mfp::from(*v)).collect() // Convert u64 to Mfp
    }

    /// Gets verifying key as `Mfp`
    pub fn get_vk(&self) -> Mfp {
        Mfp::from(self.vk) // Convert u64 to Mfp
    }
}