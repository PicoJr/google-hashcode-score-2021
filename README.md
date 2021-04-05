[![GitHub license](https://img.shields.io/github/license/PicoJr/google-hashcode-score-2021)](https://github.com/PicoJr/google-hashcode-score-2021/blob/master/LICENSE)

# Google Hashcode 2021 Score Calculator

Computes Google Hashcode 2021 Qualification Round score.

It gives the same results as Google for our submissions.

It gives the same result for the example file as well.

**Caution**: It may not be robust against incorrect submission file.

## Quickstart

### Install Rust Toolchain using rustup

cf https://www.rust-lang.org/learn/get-started

#### On Unix

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### On Windows

refer to https://rustup.rs (download and run `rustup-init.exe`)

### Compile and run

```
cargo run --release -- res/a.txt res/b.txt res/c.txt res/d.txt res/e.txt res/f.txt -o out/a.out out/b.out out/c.out out/d.out out/e.out out/f.out
```

output:

```
out/a.out score: 2,002
out/b.out score: 4,566,783
out/c.out score: 1,299,593
out/d.out score: 1,586,428
out/e.out score: 710,095
out/f.out score: 1,408,553
total score: 9,573,454
```

### Performance

cpu: `AMD Ryzen 7 3700X`

```
❯ hyperfine "./target/release/ghc2021-score res/a.txt res/b.txt res/c.txt res/d.txt res/e.txt res/f.txt -o out/a.out out/b.out out/c.out out/d.out out/e.out out/f.out"
Benchmark #1: ./target/release/ghc2021-score res/a.txt res/b.txt res/c.txt res/d.txt res/e.txt res/f.txt -o out/a.out out/b.out out/c.out out/d.out out/e.out out/f.out
  Time (mean ± σ):     275.7 ms ±   7.1 ms    [User: 256.2 ms, System: 17.7 ms]
  Range (min … max):   263.9 ms … 288.0 ms    11 runs
```
