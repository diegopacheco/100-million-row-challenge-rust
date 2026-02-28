# 100 Million Row Challenge - Rust Solution

## Problem Statement

Parse 100 million CSV rows of website visit data (`URL,datetime`) and produce a JSON file
that maps each URL path to its daily visit counts, sorted by date ascending.

## Input Format

```
https://stitcher.io/blog/some-post,2026-01-24T01:16:58+00:00
https://stitcher.io/blog/another-post,2024-01-24T01:16:58+00:00
```

## Output Format

Pretty-printed JSON with URL paths as keys and date-count maps as values:

```json
{
    "\/blog\/some-post": {
        "2025-01-24": 1,
        "2026-01-24": 2
    }
}
```

- Keys are URL paths (without the domain), with forward slashes escaped as `\/`
- Dates sorted ascending
- Pretty JSON output

## Architecture

### Generator (`src/generator.rs`)
- Predefined list of blog URL paths
- Random dates within a 3-year range (2024-2026)
- Writes CSV rows to `measurements.txt`
- Configurable row count (default 1M, supports 100M)

### Processor (`src/processor.rs`)
- Memory-mapped file for zero-copy reads
- Splits file into chunks aligned to newline boundaries
- Uses rayon for parallel chunk processing
- Each thread builds a local `HashMap<String, HashMap<String, u64>>`
- Merges all thread-local maps
- Sorts paths and dates alphabetically
- Writes pretty JSON to `output.json`

## Multi-Threading Strategy

1. Memory-map the input file
2. Determine chunk boundaries (one per CPU core), aligned to newline boundaries
3. Each thread processes its chunk independently, building a local hashmap
4. Merge all hashmaps using rayon's `reduce`
5. Sort and serialize to JSON

## Performance Considerations

- Memory-mapped I/O avoids buffered read overhead
- Parallel chunk processing saturates all CPU cores
- Thread-local hashmaps avoid contention
- Manual date extraction (first 10 chars of ISO datetime) avoids full datetime parsing
- Manual URL path extraction avoids URL parsing libraries
- BTreeMap for sorted output without explicit sort step

## Dependencies

- `rayon` for parallel processing
- `memmap2` for memory-mapped file I/O
- `serde` + `serde_json` for JSON serialization
- `rand` for data generation
