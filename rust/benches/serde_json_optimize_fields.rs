//! Benchmark the performance of deserialization by optimizing fields. The cost of this optimization is that each scenario requires defining a specific type containing only the fields needed for that scenario (rather than using a single unified type containing all information).

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use serde::Deserialize;

/// Read test JSON file
///
/// Not using `include!` to reduce the compiled file size; the corresponding JSON file is not required when this benchmark is not run.
fn read_test_json() -> std::io::Result<String> {
    let file_path = "data/go_json.json";
    let file_content = std::fs::read_to_string(file_path)?;
    Ok(file_content)
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Item {
    version: String,
    files: Vec<FileDto>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct FileDto {
    filename: String,
    os: String,
    arch: String,
    sha256: String,
    kind: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct SimplerItem {
    version: String,
}

fn criterion_benchmark(c: &mut Criterion) {
    let json_str = read_test_json().unwrap();

    c.bench_function("Deserialize Item", |b| {
        b.iter(|| {
            black_box(&serde_json::from_str::<Vec<Item>>(&json_str));
        })
    });
    c.bench_function("Deserialize SimplerItem", |b| {
        b.iter(|| {
            black_box(&serde_json::from_str::<Vec<SimplerItem>>(&json_str));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
