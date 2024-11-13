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


use anyhow::Result;
use proof_generation::main_proof_gen;
use std::arch::{asm, global_asm};

// global_asm!(
//     ".data",
//     ".global x18_array",   // Declare x18_array as a global label
//     "x18_array: .space 20" // Allocate 20 bytes for x18_array
// );

fn main() -> Result<()> {
    // // Test
    // unsafe {
    //     asm!(
    //         "addi s2, s2, 12",
    //         "addi s2, s2, 12",
    //         "addi s2, s2, 12",
    //         "addi s2, s2, 12",
    //     );
    // }

    // Initiate the proof generation process
    main_proof_gen()?;

    Ok(())
}
