use super::proof_generation::AHPData;

pub struct Verification {
    pub result: bool
}

impl Verification {
    pub fn new() -> Self {
        Self { result: false }
    }

    pub fn verify(verification_parameters: Vec<AHPData>) -> bool {
        todo!()
    }
}

