#!/bin/bash

# Read args
if [ -z "$1" ]; then
  echo "Usage: sh mysh.sh [file_name] [options]"
  exit 1
fi

find data/ -type f -not -name 'device_config.json' -delete

# setup benchmark
filename="$1"
: > "$filename"
: > "report.txt"
options="$2"
dir="debug"
if [ "$options" = "--release" ]; then 
  dir="release"
fi

# set p number 
# create a file if not exist at root


export RUSTFLAGS=""
# Build and Run
cargo build -p setup $options >> "$filename" && \
echo "Setup: =====================================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/setup >> "$filename" && \

cargo build -p commitment_generation $options >> "$filename" && \
echo "Commitment: ================================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/commitment_generation program.s data/setup3.json data/device_config.json >> "$filename" && \

./unrelated/run_proof_riscv.sh >> "$filename" && \

cargo build -p proof_verification $options >> "$filename" && \
echo "Proof Verification: ========================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/proof_verification data/program_commitment.json data/proof.json data/setup3.json >> "$filename"