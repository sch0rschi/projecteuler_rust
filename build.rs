use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    download_hand_ranks();
    scan_for_solutions();
}

fn download_hand_ranks() {
    let url = "https://raw.githubusercontent.com/christophschmalhofer/poker/master/XPokerEval/XPokerEval.TwoPlusTwo/HandRanks.dat";
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&manifest_dir).join("resources/HandRanks.dat");

    if path.exists() {
        return;
    }

    println!("HandRanks.dat not found, downloading...");

    fs::create_dir_all(path.parent().unwrap()).unwrap();

    let response = ureq::get(url).call().expect("download failed");

    let mut file = fs::File::create(path).expect("create file failed");
    let mut binding = response.into_body();
    let mut reader = binding.as_reader();

    std::io::copy(&mut reader, &mut file).expect("write failed");

    println!("download complete");
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
