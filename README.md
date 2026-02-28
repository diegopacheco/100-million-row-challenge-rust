# 100 Million Row Challenge - Rust

Rust solution for the [100 Million Row Challenge](https://github.com/tempestphp/100-million-row-challenge).
Parses 100M CSV rows of website visit data and produces a JSON file with visit counts per URL path per day.

## Input Format

```
https://stitcher.io/blog/php-enums,2024-01-24T01:16:58+00:00
https://stitcher.io/blog/11-million-rows-in-seconds,2026-01-24T01:12:11+00:00
```

## Output Format

```json
{
    "\/blog\/php-enums": {
        "2024-01-24": 1
    },
    "\/blog\/11-million-rows-in-seconds": {
        "2026-01-24": 2
    }
}
```

## How It Works

- Memory-mapped file I/O via `memmap2` for zero-copy reads
- File is split into chunks aligned to newline boundaries (one chunk per CPU core)
- Each chunk is processed in parallel using `rayon`
- Thread-local `BTreeMap` results are merged at the end
- Output is sorted alphabetically by path and date

## Build

```
cargo build --release
```

## Run

Generate data and process it:
```
./run.sh
```

Or step by step:
```
./target/release/row-challenge generate 1_000_000
./target/release/row-challenge process
```

Data files are stored in `target/data/`.

## Result

```
❯ ./run.sh
    Finished `release` profile [optimized] target(s) in 0.02s
=== Generating 100,000,000 rows ===
Generated 10000000 rows...
Generated 20000000 rows...
Generated 30000000 rows...
Generated 40000000 rows...
Generated 50000000 rows...
Generated 60000000 rows...
Generated 70000000 rows...
Generated 80000000 rows...
Generated 90000000 rows...
Generated 100000000 rows to target/data/measurements.txt

=== Processing measurements.txt ===
Processed 50 unique paths to target/data/output.json
Completed in 4.171s

=== Done ===
Output written to target/data/output.json
{
    "\/blog\/11-million-rows-in-seconds": {
        "2024-01-01": 1948,
        "2024-01-02": 1934,
        "2024-01-03": 1971,
        "2024-01-04": 1947,
        "2024-01-05": 1966,
        "2024-01-06": 1959,
        "2024-01-07": 1928,
        "2024-01-08": 1998,
        "2024-01-09": 1994,
        "2024-01-10": 1940,
        "2024-01-11": 1946,
        "2024-01-12": 2055,
        "2024-01-13": 1899,
        "2024-01-14": 1961,
        "2024-01-15": 1971,
        "2024-01-16": 2017,
        "2024-01-17": 1899,
        "2024-01-18": 1952,
```