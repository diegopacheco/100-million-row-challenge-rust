#!/bin/bash

cargo build --release

echo "=== Generating 100,000,000 rows ==="
./target/release/row-challenge generate 100_000_000

echo ""
echo "=== Processing measurements.txt ==="
./target/release/row-challenge process

echo ""
echo "=== Done ==="
echo "Output written to target/data/output.json"
head -20 target/data/output.json
