use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use memmap2::Mmap;
use rayon::prelude::*;

type VisitMap = BTreeMap<String, BTreeMap<String, u64>>;

fn parse_line(line: &[u8]) -> Option<(&str, &str)> {
    if line.is_empty() {
        return None;
    }

    let comma_pos = line.iter().rposition(|&b| b == b',')?;
    let url = std::str::from_utf8(&line[..comma_pos]).ok()?;
    let datetime = std::str::from_utf8(&line[comma_pos + 1..]).ok()?;

    let path_start = url.find("://")?;
    let path_start = url[path_start + 3..].find('/').map(|p| p + path_start + 3)?;
    let path = &url[path_start..];

    if datetime.len() < 10 {
        return None;
    }
    let date = &datetime[..10];

    Some((path, date))
}

fn process_chunk(chunk: &[u8]) -> VisitMap {
    let mut map: VisitMap = BTreeMap::new();

    let mut start = 0;
    while start < chunk.len() {
        let end = chunk[start..]
            .iter()
            .position(|&b| b == b'\n')
            .map(|p| start + p)
            .unwrap_or(chunk.len());

        let line = &chunk[start..end];
        if let Some((path, date)) = parse_line(line) {
            *map.entry(path.to_string())
                .or_default()
                .entry(date.to_string())
                .or_insert(0) += 1;
        }

        start = end + 1;
    }

    map
}

fn merge_maps(mut a: VisitMap, b: VisitMap) -> VisitMap {
    for (path, dates) in b {
        let entry = a.entry(path).or_default();
        for (date, count) in dates {
            *entry.entry(date).or_insert(0) += count;
        }
    }
    a
}

pub fn process(input_path: &str, output_path: &str) {
    let file = File::open(input_path).expect("Failed to open input file");
    let mmap = unsafe { Mmap::map(&file).expect("Failed to mmap file") };
    let data = &mmap[..];

    let num_threads = rayon::current_num_threads();
    let chunk_size = data.len() / num_threads;

    let mut boundaries = Vec::with_capacity(num_threads + 1);
    boundaries.push(0);

    for i in 1..num_threads {
        let mut pos = i * chunk_size;
        while pos < data.len() && data[pos] != b'\n' {
            pos += 1;
        }
        if pos < data.len() {
            pos += 1;
        }
        boundaries.push(pos);
    }
    boundaries.push(data.len());

    let chunks: Vec<&[u8]> = boundaries
        .windows(2)
        .map(|w| &data[w[0]..w[1]])
        .collect();

    let result = chunks
        .par_iter()
        .map(|chunk| process_chunk(chunk))
        .reduce(BTreeMap::new, merge_maps);

    let json = format_json(&result);

    let mut out = File::create(output_path).expect("Failed to create output file");
    out.write_all(json.as_bytes()).expect("Failed to write output");

    eprintln!("Processed {} unique paths to {}", result.len(), output_path);
}

fn format_json(map: &VisitMap) -> String {
    let mut out = String::from("{\n");
    let total = map.len();

    for (i, (path, dates)) in map.iter().enumerate() {
        let escaped_path = path.replace("/", "\\/");
        out.push_str(&format!("    \"{}\": {{\n", escaped_path));

        let date_total = dates.len();
        for (j, (date, count)) in dates.iter().enumerate() {
            out.push_str(&format!("        \"{}\": {}", date, count));
            if j < date_total - 1 {
                out.push(',');
            }
            out.push('\n');
        }

        out.push_str("    }");
        if i < total - 1 {
            out.push(',');
        }
        out.push('\n');
    }

    out.push('}');
    out
}
