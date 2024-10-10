use std::fs::File;
use std::io::BufWriter;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use rand::thread_rng;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_str;
use serde_json::json;

use serde_json::Value;

use crate::json_file::open_file;
use crate::json_file::store_in_json_file;
use crate::json_file::write_set;
use crate::math::kzg;
use crate::math::Mfp;
use crate::math::GENERATOR;
use crate::math::P;
use crate::to_bint;

#[derive(Debug)]
pub struct Setup {
    ck: Vec<Mfp>,
    vk: Mfp,
}

impl Setup {
    pub fn new() -> Self {
        Self {
            ck: vec![],
            vk: Mfp::default(),
        }
    }

    pub fn key_generate(&mut self, num: u64) {
        // TODO:
        let tau = thread_rng().gen_range(1..P - 1);
        // let tau = 119;

        let ck = kzg::setup(num, tau, GENERATOR);
        self.ck = ck;
        self.vk = self.ck[1];
    }

    pub fn store(&self, path: &str) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        let setup_json = SetupJson::new(&self.ck);
        serde_json::to_writer(writer, &setup_json)?;
        Ok(())
    }

    pub fn restore(path: &str) -> Result<SetupJson> {
        let reader = open_file(&PathBuf::from(path))?;
        let setup_json: SetupJson = serde_json::from_reader(reader)?;
        Ok(setup_json)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupJson {
    ck: Vec<u64>,
}

impl SetupJson {
    pub fn new(ck: &Vec<Mfp>) -> Self {
        Self {
            ck: write_set(ck),
        }
    }

    pub fn get_commitment_key(&self) -> Vec<Mfp> {
        self.ck.iter().map(|v| Mfp::from(*v)).collect()
    }

    pub fn get_verifying_key(&self) -> Mfp {
        Mfp::from(self.ck[1])
    }
}
