use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use memmap2::Mmap;

type VisitMap<'a> = HashMap<&'a str, HashMap<[u8; 10], u64>>;

fn parse_line(line: &[u8]) -> Option<(&str, [u8; 10])> {
    if line.is_empty() {
        return None;
    }

    let comma_pos = line.iter().rposition(|&b| b == b',')?;
    let url = std::str::from_utf8(&line[..comma_pos]).ok()?;
    let datetime = &line[comma_pos + 1..];

    let path_start = url.find("://")?;
    let path_start = url[path_start + 3..].find('/').map(|p| p + path_start + 3)?;
    let path = &url[path_start..];

    if datetime.len() < 10 {
        return None;
    }
    let date: [u8; 10] = datetime[..10].try_into().ok()?;

    Some((path, date))
}

fn process_chunk<'a>(chunk: &'a [u8]) -> VisitMap<'a> {
    let mut map: VisitMap<'a> = HashMap::new();

    let mut start = 0;
    while start < chunk.len() {
        let end = chunk[start..]
            .iter()
            .position(|&b| b == b'\n')
            .map(|p| start + p)
            .unwrap_or(chunk.len());

        let line = &chunk[start..end];
        if let Some((path, date)) = parse_line(line) {
            *map.entry(path)
                .or_default()
                .entry(date)
                .or_insert(0) += 1;
        }

        start = end + 1;
    }

    map
}

fn merge_maps<'a>(mut a: VisitMap<'a>, b: VisitMap<'a>) -> VisitMap<'a> {
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

    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
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

    let result = std::thread::scope(|s| {
        let handles: Vec<_> = chunks
            .iter()
            .map(|chunk| s.spawn(|| process_chunk(chunk)))
            .collect();

        handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .reduce(merge_maps)
            .unwrap_or_default()
    });

    write_json(&result, output_path);

    eprintln!("Processed {} unique paths to {}", result.len(), output_path);
}

fn write_json(map: &VisitMap, path: &str) {
    let file = File::create(path).expect("Failed to create output file");
    let mut out = BufWriter::with_capacity(1024 * 1024, file);

    let mut paths: Vec<&&str> = map.keys().collect();
    paths.sort();
    let total = paths.len();

    out.write_all(b"{\n").unwrap();

    for (i, path) in paths.iter().enumerate() {
        let escaped_path = path.replace("/", "\\/");
        write!(out, "    \"{}\": {{\n", escaped_path).unwrap();

        let dates_map = &map[**path];
        let mut dates: Vec<(&[u8; 10], &u64)> = dates_map.iter().collect();
        dates.sort_by_key(|(d, _)| *d);
        let date_total = dates.len();

        for (j, (date, count)) in dates.iter().enumerate() {
            let date_str = std::str::from_utf8(date.as_slice()).unwrap();
            write!(out, "        \"{}\": {}", date_str, count).unwrap();
            if j < date_total - 1 {
                out.write_all(b",").unwrap();
            }
            out.write_all(b"\n").unwrap();
        }

        out.write_all(b"    }").unwrap();
        if i < total - 1 {
            out.write_all(b",").unwrap();
        }
        out.write_all(b"\n").unwrap();
    }

    out.write_all(b"}").unwrap();
    out.flush().unwrap();
}
