use std::fs::File;
use std::io::{BufWriter, Write};
use rand::Rng;

const PATHS: &[&str] = &[
    "/blog/php-enums",
    "/blog/11-million-rows-in-seconds",
    "/blog/laravel-beyond-crud",
    "/blog/php-81-enums",
    "/blog/a-project-at-stitcher",
    "/blog/php-what-i-dont-like",
    "/blog/new-in-php-81",
    "/blog/new-in-php-82",
    "/blog/new-in-php-83",
    "/blog/new-in-php-84",
    "/blog/generics-in-php",
    "/blog/readonly-classes-in-php-82",
    "/blog/fibers-with-a-grain-of-salt",
    "/blog/php-enum-style-guide",
    "/blog/constructor-promotion-in-php-8",
    "/blog/php-match-or-switch",
    "/blog/named-arguments-in-php-80",
    "/blog/php-enums-and-static-analysis",
    "/blog/short-closures-in-php",
    "/blog/attributes-in-php-8",
    "/blog/typed-properties-in-php-74",
    "/blog/a-letter-to-the-php-community",
    "/blog/union-types-in-php-80",
    "/blog/what-is-new-in-php",
    "/blog/readonly-properties-in-php-82",
    "/blog/nullsafe-operator-in-php",
    "/blog/php-deprecations-84",
    "/blog/property-hooks-in-php-84",
    "/blog/asymmetric-visibility-in-php-84",
    "/blog/crafting-quality-code",
    "/blog/object-oriented-programming",
    "/blog/design-patterns-explained",
    "/blog/functional-programming-in-php",
    "/blog/testing-best-practices",
    "/blog/clean-architecture",
    "/blog/domain-driven-design",
    "/blog/event-sourcing-patterns",
    "/blog/cqrs-explained",
    "/blog/microservices-patterns",
    "/blog/api-design-principles",
    "/blog/rest-vs-graphql",
    "/blog/database-optimization",
    "/blog/caching-strategies",
    "/blog/security-best-practices",
    "/blog/ci-cd-pipelines",
    "/blog/docker-for-developers",
    "/blog/kubernetes-basics",
    "/blog/serverless-architecture",
    "/blog/web-performance-tips",
    "/blog/frontend-frameworks-comparison",
];

const DOMAIN: &str = "https://stitcher.io";

pub fn generate(path: &str, count: usize) {
    let file = File::create(path).expect("Failed to create output file");
    let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, file);
    let mut rng = rand::thread_rng();

    let years = [2024, 2025, 2026];

    for i in 0..count {
        let path_idx = rng.gen_range(0..PATHS.len());
        let year = years[rng.gen_range(0..years.len())];
        let month = rng.gen_range(1..=12u32);
        let day = rng.gen_range(1..=28u32);
        let hour = rng.gen_range(0..24u32);
        let minute = rng.gen_range(0..60u32);
        let second = rng.gen_range(0..60u32);

        write!(
            writer,
            "{}{},{:04}-{:02}-{:02}T{:02}:{:02}:{:02}+00:00\n",
            DOMAIN, PATHS[path_idx], year, month, day, hour, minute, second
        )
        .expect("Failed to write");

        if i % 10_000_000 == 0 && i > 0 {
            eprintln!("Generated {} rows...", i);
        }
    }

    writer.flush().expect("Failed to flush");
    eprintln!("Generated {} rows to {}", count, path);
}
