#!/bin/bash
# read args
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

# Build and Run
cargo build -p setup $options >> "$filename" && \
echo "Setup: =====================================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/setup >> "$filename" && \

cargo build -p commitment_generation $options >> "$filename" && \
echo "Commitment: ================================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/commitment_generation program.s data/setuptest.json data/device_config.json >> "$filename" && \

# This must be compiled with the local toolchain configuration
cd proof_generation 
cargo build $options >> "$filename" && \
cd ../ && \
echo "Proof Generation: ==========================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/proof_generation data/setuptest.json data/program_commitment.json data/program_params.json >> "$filename" && \

# ./unrelated/run_proof_riscv.sh >> "$filename"

cargo build -p proof_verification $options >> "$filename" && \
echo "Proof Verification: ========================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/proof_verification data/program_commitment.json data/proof.json data/setuptest.json >> "$filename"