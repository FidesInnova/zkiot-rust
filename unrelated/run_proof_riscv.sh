#!/bin/bash

cd proof_generation && \
# Set the Rust compiler flags to specify the linker for RISC-V target
export RUSTFLAGS="-C linker=riscv64-linux-gnu-gcc"
# Build the project for the RISC-V architecture
cargo build --target riscv64gc-unknown-linux-gnu

cd ../
# Set the QEMU linker prefix for running the compiled binary
export QEMU_LD_PREFIX=/usr/riscv64-linux-gnu

# Execute the compiled proof_generation binary using QEMU for RISC-V
qemu-riscv64 target/riscv64gc-unknown-linux-gnu/debug/proof_generation

# Reset flag
export RUSTFLAGS=""