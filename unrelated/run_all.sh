#!/bin/bash

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

# Read args
if [ "$#" -lt 2 ]; then
  echo "Usage: sh mysh.sh [file_name] [setup number] [options]"
  exit 1
fi

find data/ -type f -not -name 'device_config.json' -delete

# setup benchmark
filename="$1"
: > "$filename"
: > "report.txt"
options="$3"
dir="debug"
if [ "$options" = "--release" ]; then 
  dir="release"
fi

# set p number
./unrelated/z_vec 5087281

export RUSTFLAGS=""
# Build and Run
cargo build -p setup $options >> "$filename" && \
echo "Setup: =====================================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/setup >> "$filename" && \

cargo build -p commitment_generation $options >> "$filename" && \
echo "Commitment: ================================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/commitment_generation program.s data/setup$2.json data/device_config.json >> "$filename" && \


# ./unrelated/run_proof_riscv.sh $2 >> "$filename" && \
cargo build -p proof_generation $options >> "$filename" && \
echo "Proof Generation: ================================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/proof_generation data/setup$2.json >> "$filename" && \


cargo build -p proof_verification $options >> "$filename" && \
echo "Proof Verification: ========================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/proof_verification data/program_commitment.json data/proof.json data/setup$2.json >> "$filename"