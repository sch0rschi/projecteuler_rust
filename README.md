# projecteuler_rust

Rust solutions to [Project Euler](https://projecteuler.net/) problems 0–100.

![Project Euler Profile](https://projecteuler.net/profile/sch0rschi.png)

---

## Structure

```
src/
├── main.rs          # benchmark runner
├── lib.rs           # generated module registry
├── libs/            # shared utilities
└── solutions/       # one file per problem
    ├── problem_0001.rs
    ├── problem_0002.rs
    └── ...
```

Modules and the benchmark registry are auto-generated via `build.rs`, which also downloads `HandRanks.dat` if missing.

---

## Requirements

- Rust (latest stable)

---

## Running

Verify all solutions (via Rust unit tests):

```bash
cargo test --release
```

Benchmark all solutions:

```bash
cargo run --release
```

---

## Performance

Benchmarks were run on a **MacBook Air M3** using Rust `--release` mode with 10 warmup iterations and 10 measured
iterations per problem.

### Runtime Highlights

- **Problems 0–50:** `~11ms`
- **Problems 51–100:** `~186ms`
- **Total runtime (all 101 problems):** `~197ms`

---

### Fastest Solutions

Several problems complete in effectively constant time (`~0–50ns`), including:

- `0001` Multiples of 3 and 5
- `0006` Sum square difference
- `0015` Lattice paths
- `0024` Lexicographic permutations
- `0100` Arranged probability

---

### Slowest Solutions

| Problem | Avg Runtime |
|---------|------------:|
| 0060    |  `64.532ms` |
| 0070    |  `46.691ms` |
| 0095    |  `17.814ms` |
| 0073    |  `16.750ms` |
| 0058    |   `5.471ms` |
| 0078    |   `4.750ms` |
| 0055    |   `2.948ms` |
| 0069    |   `2.857ms` |
| 0072    |   `2.336ms` |
| 0074    |   `2.308ms` |

---

### Notes

- The entire first half of the problems executes in `~11ms`.
- Most runtime-heavy problems involve:
    - prime sieving
    - combinatorial search
    - graph/path exploration
    - dynamic programming
- Even the slowest solution completes in `~65ms`.

---

## License

Project Euler problems are licensed under [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/).
