use projecteuler::SOLUTIONS;
use std::hint::black_box;
use std::io::{self, Write};
use std::time::{Duration, Instant};

struct ProblemResult {
    name: &'static str,
    min: Duration,
    avg: Duration,
    max: Duration,
}

fn main() {
    const WARMUP: usize = 100;
    const ITERATIONS: usize = 100;

    let total_tasks = SOLUTIONS.len();
    let mut results: Vec<ProblemResult> = Vec::new();

    // =========================================================
    // WARMUP
    // =========================================================
    println!("Warmup starting...");

    for (i, &(_, f)) in SOLUTIONS.iter().enumerate() {
        for r in 0..WARMUP {
            let _: () = f();
            black_box(());

            print_progress("Warmup", i * WARMUP + r + 1, total_tasks * WARMUP);
        }
    }

    clear_line();
    println!("Warmup complete.");

    // =========================================================
    // BENCHMARK
    // =========================================================
    println!();
    println!("Benchmark starting...");
    print_table_header();

    for (i, &(name, f)) in SOLUTIONS.iter().enumerate() {
        let mut times = Vec::with_capacity(ITERATIONS);

        for r in 0..ITERATIONS {
            let start = Instant::now();
            let _: () = f();
            black_box(());
            times.push(start.elapsed());

            print_progress(
                "Benchmark",
                i * ITERATIONS + r + 1,
                total_tasks * ITERATIONS,
            );
        }

        clear_line();

        let min = *times.iter().min().unwrap();
        let max = *times.iter().max().unwrap();
        let avg = times.iter().sum::<Duration>() / ITERATIONS as u32;

        let result = ProblemResult {
            name,
            min,
            avg,
            max,
        };

        print_result_line(&result);
        results.push(result);

        print_progress("Bench", (i + 1) * ITERATIONS, total_tasks * ITERATIONS);
    }

    clear_line();
    println!("Benchmark complete.");

    // =========================================================
    // SPLIT STATS
    // =========================================================
    let mid = results.len() / 2;

    let first: Duration = results[..mid].iter().map(|r| r.avg).sum();
    let second: Duration = results[mid..].iter().map(|r| r.avg).sum();
    let total: Duration = results.iter().map(|r| r.avg).sum();

    println!();
    println!("Split statistics:");
    println!("0–50   : {}", format_duration(first));
    println!("51–100 : {}", format_duration(second));
    println!("Total  : {}", format_duration(total));

    // =========================================================
    // SLOWEST 10 (0-50)
    // =========================================================
    println!();
    println!("Top 10 slowest (0-50):");

    let mut fastest: Vec<&ProblemResult> = results.iter().take(50).collect();
    fastest.sort_by(|a, b| a.avg.cmp(&b.avg).reverse());

    print_table_header();
    print_top_n(&fastest, 10);

    // =========================================================
    // SLOWEST 10 (0-100)
    // =========================================================
    println!();
    println!("Top 10 slowest (0-100):");

    let mut slowest: Vec<&ProblemResult> = results.iter().collect();
    slowest.sort_by(|a, b| a.avg.cmp(&b.avg).reverse());

    print_table_header();
    print_top_n(&slowest, 10);
}

// =========================================================
// HELPERS
// =========================================================

fn print_top_n(list: &[&ProblemResult], n: usize) {
    for r in list.iter().take(n) {
        print_result_line(r);
    }
}

fn print_result_line(r: &ProblemResult) {
    println!(
        "{:<6} {:>12} {:>12} {:>12}",
        r.name,
        format_duration(r.min),
        format_duration(r.avg),
        format_duration(r.max),
    );
}

fn print_table_header() {
    println!("{:<6} {:>12} {:>12} {:>12}", "Prob", "Min", "Avg", "Max");
    println!("{}", "-".repeat(45));
}

fn print_progress(label: &str, current: usize, total: usize) {
    let progress = current as f32 / total as f32;
    let percent = progress * 100.0;

    let width = 30;
    let filled = (progress * width as f32) as usize;

    let bar = format!("[{}{}]", "#".repeat(filled), ".".repeat(width - filled));

    print!("\r{} {} {:5.1}%", label, bar, percent);
    io::stdout().flush().unwrap();
}

fn clear_line() {
    print!("\r\x1b[2K");
    io::stdout().flush().unwrap();
}

fn format_duration(d: Duration) -> String {
    let ns = d.as_nanos();

    if ns < 1_000 {
        format!("{:>7.3} ns", ns as f64)
    } else if ns < 1_000_000 {
        format!("{:>7.3} µs", ns as f64 / 1_000.0)
    } else if ns < 1_000_000_000 {
        format!("{:>7.3} ms", ns as f64 / 1_000_000.0)
    } else {
        format!("{:>7.3} s", d.as_secs_f64())
    }
}
