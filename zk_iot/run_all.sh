#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: sh mysh.sh [file_name]"
  exit 1
fi

FILENAME="$1"

echo "" > "$FILENAME" && \
cargo run -p setup --release >> "$FILENAME" && \
cargo run -p commitment --release >> "$FILENAME" && \
cargo run -p proof_generation --release >> "$FILENAME" && \
cargo run -p proof_verification --release >> "$FILENAME"
