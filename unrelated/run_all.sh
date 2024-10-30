#!/bin/bash
# read args
if [ -z "$1" ]; then
  echo "Usage: sh mysh.sh [file_name] [options]"
  exit 1
fi


# setup benchmark
filename="$1"
: > "$filename"
: > "report.txt"
options="$2"
dir="debug"
if [ "$options" = "--release" ]; then 
  dir="release"
fi

# run
cargo build -p setup $options >> "$filename" && \
echo "Setup: =====================================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/setup >> "$filename" && \

cargo build -p commitment_generation $options >> "$filename" && \
echo "Commitment: ================================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/commitment_generation program.s zkp_data/device_config.json zkp_data/setup.json >> "$filename" && \

cargo build -p proof_generation $options >> "$filename" && \
echo "Proof Generation: ==========================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/proof_generation zkp_data/setup.json zkp_data/program_commitment.json zkp_data/program_params.json zkp_data/device_config.json >> "$filename" && \

cargo build -p proof_verification $options >> "$filename" && \
echo "Proof Verification: ========================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/proof_verification zkp_data/program_commitment.json zkp_data/proof.json >> "$filename"