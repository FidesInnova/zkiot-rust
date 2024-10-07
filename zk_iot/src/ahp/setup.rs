use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use serde_json::from_str;
use serde_json::json;

use serde_json::Value;

use crate::json_file::open_file;
use crate::json_file::store_in_json_file;
use crate::json_file::write_set;
use crate::math::kzg;
use crate::math::Mfp;
use crate::math::GENERATOR;
use crate::to_bint;

#[derive(Debug)]
pub struct Setup {
    ck: Vec<Mfp>,
    vk: Mfp
}

impl Setup {
    pub fn new() -> Self {
        Self {
            ck: vec![],
            vk: Mfp::default()
        }
    }

    pub fn key_generate(&mut self) {
        // TODO: Define a random value b within the range F(0..P-n) and ensure 0 < b <= P - n
        // let b = thread_rng().gen_range(0..50);

        let tau = 119;
        let ck = kzg::setup(10_000, tau, GENERATOR);
        self.ck = ck;
        self.vk = self.ck[1];
    }

    pub fn store(&self, path: &str) -> Result<()> {
        let json_value = json!({
            "ck": write_set(&self.ck),
            "vk": to_bint!(self.vk)
        });
        
        store_in_json_file(json_value, path)
    }

    
    pub fn restore(path: &str) -> Result<(Vec<Mfp>, Mfp)> {
        // Read the JSON file
        let mut reader = open_file(&PathBuf::from(path))?;
        // Read the contents into a String
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;

        // Parse the JSON data
        let json_value: Value = from_str(&contents)?;
        // Extract and convert the "ck"
        let ck: Vec<u64> = serde_json::from_value(json_value["ck"].clone())?;
        let ck: Vec<Mfp> = ck.iter().map(|v| Mfp::from(*v)).collect();
        let vk = Mfp::from(ck[1]);
        Ok((ck.clone(), vk))
    }
}