# projecteuler_rust

Rust solutions to [Project Euler](https://projecteuler.net/) problems 1–100.

![Project Euler Profile](https://projecteuler.net/profile/sch0rschi.png)

---

## Structure

Each problem is an independent binary under `src/bin/`:

```
src/bin/
├── problem_0001.rs
├── problem_0002.rs
├── problem_0003.rs
└── ...
```

Shared utilities live in `src/lib.rs`.

---

## Requirements

- Rust (latest stable)
- Bash (for `test.sh`)

Some problems use a precomputed poker hand-rank table (`HandRanks.dat`), which `test.sh` downloads automatically if missing.

---

## Running

Build and run a single problem:

```bash
cargo run --release --bin problem_0042
```

Run and verify all problems:

```bash
./test.sh
```

`test.sh` lints with `cargo clippy`, builds in release mode, then executes every binary in parallel and reports pass/fail.

---

## License

Project Euler problems are licensed under [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/).