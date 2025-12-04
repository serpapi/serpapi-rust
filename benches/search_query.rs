//! Benchmark suite for SerpAPI search queries
//!
//! This benchmark executes the same search query 10 times and measures:
//! - Runtime performance
//! - Memory usage
//! - CPU performance (JSON parsing)
//!
//! Usage:
//!   export SERPAPI_KEY=your_api_key
//!   cargo bench --bench search_query

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serpapi::serpapi::Client;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

fn api_key() -> String {
    std::env::var("SERPAPI_KEY").unwrap_or_else(|_| {
        eprintln!("Warning: SERPAPI_KEY not set. Benchmarks may fail.");
        "".to_string()
    })
}

fn setup_client() -> Client {
    let mut default = HashMap::<String, String>::new();
    default.insert("engine".to_string(), "google".to_string());
    default.insert("api_key".to_string(), api_key());
    Client::new(default).unwrap()
}

fn setup_search_params() -> HashMap<String, String> {
    let mut parameter = HashMap::<String, String>::new();
    parameter.insert("q".to_string(), "coffee".to_string());
    parameter.insert(
        "location".to_string(),
        "Austin, TX, Texas, United States".to_string(),
    );
    parameter
}

fn benchmark_search_query(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let client = Arc::new(setup_client());
    let params = setup_search_params();

    // Collect memory statistics separately using Arc<Mutex<>>
    let memory_deltas: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));
    let peak_memory: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    let mut group = c.benchmark_group("search_query");
    group.sample_size(10); // Execute 10 times
    group.measurement_time(std::time::Duration::from_secs(120)); // Allow up to 120s for API calls

    let client_clone = Arc::clone(&client);
    let params_clone = params.clone();
    let memory_deltas_clone = Arc::clone(&memory_deltas);
    let peak_memory_clone = Arc::clone(&peak_memory);

    group.bench_function("search_with_parsing", |b| {
        let client = Arc::clone(&client_clone);
        let params = params_clone.clone();
        let memory_deltas = Arc::clone(&memory_deltas_clone);
        let peak_memory = Arc::clone(&peak_memory_clone);

        b.to_async(&rt).iter(|| {
            let client = Arc::clone(&client);
            let params = params.clone();
            let memory_deltas = Arc::clone(&memory_deltas);
            let peak_memory = Arc::clone(&peak_memory);

            async move {
                // Track memory before
                let mem_before = memory_stats::memory_stats()
                    .map(|m| m.physical_mem)
                    .unwrap_or(0);

                // Perform search using the shared client
                let result = client.search(black_box(params)).await;

                // CPU-intensive operation: parse and access JSON
                if let Ok(json) = result {
                    let _organic_results = json["organic_results"].as_array();
                    let _local_results = json["local_results"]["places"].as_array();
                    let _search_metadata = json["search_metadata"].as_object();

                    // Track memory after
                    let mem_after = memory_stats::memory_stats()
                        .map(|m| m.physical_mem)
                        .unwrap_or(0);

                    let mem_delta = mem_after.saturating_sub(mem_before);

                    // Store memory statistics
                    if let Ok(mut deltas) = memory_deltas.lock() {
                        deltas.push(mem_delta);
                    }
                    if let Ok(mut peak) = peak_memory.lock() {
                        *peak = (*peak).max(mem_after);
                    }

                    black_box((json, mem_delta))
                } else {
                    black_box((serde_json::Value::Null, 0usize))
                }
            }
        });
    });

    group.finish();

    // Report memory statistics
    let deltas = memory_deltas.lock().unwrap();
    let peak = peak_memory.lock().unwrap();

    if !deltas.is_empty() {
        let mean_delta: f64 = deltas.iter().sum::<usize>() as f64 / deltas.len() as f64;
        let variance: f64 = deltas
            .iter()
            .map(|&x| {
                let diff = x as f64 - mean_delta;
                diff * diff
            })
            .sum::<f64>()
            / deltas.len() as f64;
        let std_dev = variance.sqrt();

        println!("\n=== Memory Usage Statistics ===");
        println!(
            "Memory Delta (mean): {:.2} KB ({:.2} MB)",
            mean_delta / 1024.0,
            mean_delta / (1024.0 * 1024.0)
        );
        println!(
            "Memory Delta (std dev): {:.2} KB ({:.2} MB)",
            std_dev / 1024.0,
            std_dev / (1024.0 * 1024.0)
        );
        println!(
            "Peak Memory: {:.2} KB ({:.2} MB)",
            *peak as f64 / 1024.0,
            *peak as f64 / (1024.0 * 1024.0)
        );
        println!("Samples: {}", deltas.len());
        println!("===============================\n");
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .with_plots()
        .with_output_color(true);
    targets = benchmark_search_query
}

criterion_main!(benches);
