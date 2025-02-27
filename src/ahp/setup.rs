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
use rand::{thread_rng, Rng};
use serde::Serialize;
use std::io::BufWriter;
use serde::Deserialize;

use crate::kzg;
use crate::json_file::write_set;
use crate::utils::read_json_file;

/// Struct for setup data with commitment and verifying keys
pub struct Setup {
    ck: Vec<u64>, // Commitment keys
    vk: u64,      // Verifying key
}

impl Setup {
    /// Creates a new `Setup` with default keys
    pub fn default() -> Self {
        Self {
            ck: Vec::default(),
            vk: u64::default(),
        }
    }
    
    /// Generates commitment and verifying keys
    ///
    /// # Parameters
    /// - `num`: Number of keys to generate.
    pub fn generate_keys(&mut self, num: u64, p: u64, g: u64) {
        let tau = thread_rng().gen_range(1..p);  // Placeholder for a random number

        // Generate commitment keys using KZG.
        let ck = kzg::setup(num, tau, g, p);

        self.ck = ck; // Store commitment keys
        self.vk = self.ck[1]; // Set verifying key
    }

    /// Saves setup data to a JSON file
    ///
    /// # Parameters
    /// - `path`: File path to save the JSON
    pub fn store(&self, path: &str, class_number: u8) -> Result<()> {
        let file = File::create(path)?; // Create or truncate the file
        let writer = BufWriter::new(file); // Buffer for writing

        let setup_json = SetupJson::new(&self.ck, class_number); // Create JSON representation
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
    pub fn new(ck: &Vec<u64>, class: u8) -> Self {
        let ck = write_set(ck); // Convert u64 to u64
        Self {
            class,
            ck: ck.clone(), // Store commitment keys
            vk: ck[1],     // Set verifying key
        }
    }

    /// Gets commitment keys as `u64`.
    pub fn get_ck(&self) -> Vec<u64> {
        self.ck.clone()
    }

    /// Gets verifying key as `u64`
    pub fn get_vk(&self) -> u64 {
        self.vk
    }
}