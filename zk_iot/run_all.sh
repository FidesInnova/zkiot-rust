echo "" > t.txt && \
cargo run -p setup >> t.txt && \
cargo run -p proof_generation   >> t.txt && \
cargo run -p proof_verification >> t.txt