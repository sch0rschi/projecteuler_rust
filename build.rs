use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    scan_for_solutions();
}

fn scan_for_solutions() {
    let solutions_dir = "src/solutions";
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("solutions.rs");
    let mut f = fs::File::create(&dest).unwrap();

    let mut numbers: Vec<String> = fs::read_dir(solutions_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let name = entry.file_name().into_string().unwrap();
            if name.starts_with("problem_") && name.ends_with(".rs") {
                Some(name[8..12].to_string())
            } else {
                None
            }
        })
        .collect();

    numbers.sort();

    for num in &numbers {
        let abs_path = std::fs::canonicalize(format!("{solutions_dir}/problem_{num}.rs")).unwrap();
        writeln!(f, "#[path = \"{}\"]", abs_path.display()).unwrap();
        writeln!(f, "pub mod problem_{num};").unwrap();
    }

    writeln!(f, "pub const SOLUTIONS: &[(&str, fn())] = &[").unwrap();
    for num in &numbers {
        writeln!(
            f,
            "    (\"{num}\", || {{ std::hint::black_box(problem_{num}::solve_{num}()); }}),",
        )
        .unwrap();
    }
    writeln!(f, "];").unwrap();

    println!("cargo:rerun-if-changed={}", solutions_dir);
}
