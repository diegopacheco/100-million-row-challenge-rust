mod generator;
mod processor;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <generate|process> [options]", args[0]);
        eprintln!("  generate [count]  - Generate test data (default: 1000000)");
        eprintln!("  process           - Process measurements.txt -> output.json");
        std::process::exit(1);
    }

    let data_dir = "target/data";
    std::fs::create_dir_all(data_dir).expect("Failed to create target/data directory");
    let input_file = format!("{}/measurements.txt", data_dir);
    let output_file = format!("{}/output.json", data_dir);

    match args[1].as_str() {
        "generate" => {
            let count = args
                .get(2)
                .and_then(|s| s.replace("_", "").parse::<usize>().ok())
                .unwrap_or(1_000_000);
            generator::generate(&input_file, count);
        }
        "process" => {
            let start = std::time::Instant::now();
            processor::process(&input_file, &output_file);
            let elapsed = start.elapsed();
            eprintln!("Completed in {:.3}s", elapsed.as_secs_f64());
        }
        other => {
            eprintln!("Unknown command: {}", other);
            std::process::exit(1);
        }
    }
}
