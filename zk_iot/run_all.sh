#!/bin/bash
# read args
if [ -z "$1" ]; then
  echo "Usage: sh mysh.sh [file_name] [options]"
  exit 1
fi

# write number in line_num
input_file="sample.txt"
output_file="line_num.txt"
perl -0777 -i -pe 's/\n+$//g' "$input_file"
line_count=$(wc -l < "$input_file")
line_count=$((line_count + 1))
: > "$output_file"
i=1
while [ $i -le $line_count ]; do
    echo "$i" >> "$output_file"
    i=$((i + 1))
done


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
/usr/bin/time -v -a -o "report.txt" ./target/$dir/commitment_generation >> "$filename" && \

cargo build -p proof_generation $options >> "$filename" && \
echo "Proof Generation: ==========================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/proof_generation >> "$filename" && \

cargo build -p proof_verification $options >> "$filename" && \
echo "Proof Verification: ========================================" >> "report.txt" && \
/usr/bin/time -v -a -o "report.txt" ./target/$dir/proof_verification >> "$filename"