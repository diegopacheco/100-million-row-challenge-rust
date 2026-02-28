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
- Each chunk is processed in parallel using `std::thread::scope` (scoped threads)
- `ahash::AHashMap` instead of std HashMap (non-cryptographic hash, much faster for this use case)
- `memchr` crate for SIMD-accelerated newline and comma scanning
- Pre-allocated HashMap capacity (64 for paths, 1024 for dates) to avoid rehashing
- Thread-local results are merged at the end
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
    Finished `release` profile [optimized] target(s) in 0.01s
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
Completed in 1.031s

=== Done ===
Output written to target/data/output.json
{
    "\/blog\/11-million-rows-in-seconds": {
        "2024-01-01": 1992,
        "2024-01-02": 1966,
        "2024-01-03": 1997,
        "2024-01-04": 1983,
        "2024-01-05": 2024,
        "2024-01-06": 1920,
        "2024-01-07": 1918,
        "2024-01-08": 2015,
        "2024-01-09": 2000,
        "2024-01-10": 1981,
        "2024-01-11": 2035,
        "2024-01-12": 1944,
        "2024-01-13": 1982,
        "2024-01-14": 2004,
        "2024-01-15": 2025,
        "2024-01-16": 1998,
        "2024-01-17": 1971,
        "2024-01-18": 1975,
```

### Related POC

* 100MRC Rust -> https://github.com/diegopacheco/100-million-row-challenge-rust
* 100MRC Zig -> https://github.com/diegopacheco/100-million-row-challenge-zig
* 1000RC Java 25 -> https://github.com/diegopacheco/100-million-row-challenge-java

### Comparison

```

  ┌───────────────────┬───────────────────────────────┬────────────────────────────┬──────────────────────────────────────────┐
  │      Aspect       │              Zig              │            Rust            │                 Java 25                  │
  ├───────────────────┼───────────────────────────────┼────────────────────────────┼──────────────────────────────────────────┤
  │ Time              │ 0.765s                        │ 1.031s                     │ 2.063s                                   │
  ├───────────────────┼───────────────────────────────┼────────────────────────────┼──────────────────────────────────────────┤
  │ Throughput        │ ~130.7M rows/s                │ ~97.0M rows/s              │ ~48.5M rows/s                            │
  ├───────────────────┼───────────────────────────────┼────────────────────────────┼──────────────────────────────────────────┤
  │ vs Fastest        │ 1.0x (baseline)               │ 1.35x slower               │ 2.70x slower                             │
  ├───────────────────┼───────────────────────────────┼────────────────────────────┼──────────────────────────────────────────┤
  │ I/O               │ posix.mmap (direct)           │ memmap2 crate (mmap)       │ MemorySegment + Foreign API (mmap)       │
  ├───────────────────┼───────────────────────────────┼────────────────────────────┼──────────────────────────────────────────┤
  │ Parallelism       │ std.Thread.spawn (OS threads) │ std::thread::scope (scoped │ Virtual threads                          │
  │                   │                               │  threads)                  │ (newVirtualThreadPerTaskExecutor)        │
  ├───────────────────┼───────────────────────────────┼────────────────────────────┼──────────────────────────────────────────┤
  │ Map type          │ StringArrayHashMap (flat      │ HashMap (std)              │ HashMap<Long, HashMap<Long, Long>>       │
  │                   │ array)                        │                            │                                          │
  ├───────────────────┼───────────────────────────────┼────────────────────────────┼──────────────────────────────────────────┤
  │ Path key          │ []const u8 slice (zero-copy   │ &str (borrowed from mmap)  │ long hash (custom encodePath)            │
  │                   │ from mmap)                    │                            │                                          │
  ├───────────────────┼───────────────────────────────┼────────────────────────────┼──────────────────────────────────────────┤
  │ Date key          │ [10]u8 (fixed array)          │ [u8; 10] (fixed array)     │ long (encoded YYYYMMDD)                  │
  ├───────────────────┼───────────────────────────────┼────────────────────────────┼──────────────────────────────────────────┤
  │ String alloc      │ Zero (slices into mmap)       │ Zero (borrows from mmap)   │ Zero (byte[] + long encoding)            │
  │ during parse      │                               │                            │                                          │
  ├───────────────────┼───────────────────────────────┼────────────────────────────┼──────────────────────────────────────────┤
  │ Build             │ -Doptimize=ReleaseFast        │ opt-level=3, lto=true,     │ None (plain javac)                       │
  │ optimization      │                               │ codegen-units=1            │                                          │
  ├───────────────────┼───────────────────────────────┼────────────────────────────┼──────────────────────────────────────────┤
  │ External deps     │ 0                             │ 2 (memmap2, rand)          │ 0                                        │
  ├───────────────────┼───────────────────────────────┼────────────────────────────┼──────────────────────────────────────────┤
  │ Lines of code     │ ~150                          │ ~120                       │ ~180                                     │
  │ (processor)       │                               │                            │                                          │
  └───────────────────┴───────────────────────────────┴────────────────────────────┴──────────────────────────────────────────┘
```
