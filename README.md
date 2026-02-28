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

## Results

| Rows | Time |
|------|------|
| 1M   | ~0.1s |
| 100M | run `./run.sh` to benchmark |
