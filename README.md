# Google Hashcode 2021 Score Calculator

Computes Google Hashcode 2021 Qualification Round score.

It gives the same results as Google for our submissions.

It gives the sames result for the example file as well.

Caution: It may not be robust against incorrect submission file.

## Quickstart

example:

```
cargo run --release -- res/a.txt res/b.txt res/c.txt res/d.txt res/e.txt res/f.txt -o out/a.out out/b.out out/c.out out/d.out out/e.out out/f.out
```

example output:

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
  Time (mean ± σ):      1.345 s ±  0.011 s    [User: 1.323 s, System: 0.019 s]
  Range (min … max):    1.321 s …  1.359 s    10 runs
```
