#!/bin/bash
# Build and run on RISC-V emulator

# Copyright 2024 Fidesinnova, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# Set the Rust compiler flags to specify the linker for RISC-V target
export RUSTFLAGS="-C linker=riscv64-linux-gnu-gcc"
# Set the QEMU linker prefix for running the compiled binary
export QEMU_LD_PREFIX=/usr/riscv64-linux-gnu


cd proof_generation && \
# Build the project for the RISC-V architecture
cargo build --target riscv64gc-unknown-linux-gnu --release && \
cargo rustc --bin proof_generation --target riscv64gc-unknown-linux-gnu -- --emit=asm && \

cd ../ 
# Execute the compiled proof_generation binary using QEMU for RISC-V
qemu-riscv64 target/riscv64gc-unknown-linux-gnu/debug/proof_generation data/setup$1.json && \ 


# Reset flag
export RUSTFLAGS=""