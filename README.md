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

- **Total runtime (all 101 problems):** `~636ms`
- **Problems 0–50:** `~29ms`
- **Problems 51–100:** `~635ms`

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
| 0088    |  `72.239ms` |
| 0069    |  `67.150ms` |
| 0072    |  `66.268ms` |
| 0060    |  `62.073ms` |
| 0092    |  `61.664ms` |
| 0073    |  `59.909ms` |
| 0074    |  `47.576ms` |
| 0070    |  `45.518ms` |
| 0071    |  `30.531ms` |
| 0095    |  `17.589ms` |

---

### Notes

- The entire first half of the problems executes in under `30ms`.
- Most runtime-heavy problems involve:
    - prime sieving
    - combinatorial search
    - graph/path exploration
    - dynamic programming
- Even the slowest solution completes in under `72ms`.

---

## License

Project Euler problems are licensed under [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/).
