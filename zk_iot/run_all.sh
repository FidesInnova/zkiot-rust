#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: sh mysh.sh [file_name]"
  exit 1
fi

FILENAME="$1"

echo "" > "$FILENAME" && \
cargo run -p setup >> "$FILENAME" && \
cargo run -p commitment >> "$FILENAME" && \
cargo run -p proof_generation >> "$FILENAME" && \
cargo run -p proof_verification >> "$FILENAME"
