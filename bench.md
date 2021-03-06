# Benchmarks

Optimizations in chronological order.

> CPU: Intel Celeron N2840 (2) @ 2.582GHz 

## First version (6533d28)

| Command | Mean [s] |
|:---|---:|
| `./target/release/ghc2021-score res/a.txt res/b.txt res/c.txt res/d.txt res/e.txt res/f.txt -o out/a.out out/b.out out/c.out out/d.out out/e.out out/f.out` | ~110s |

## Use FxHashMap (cf5bf28)

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/ghc2021-score res/a.txt res/b.txt res/c.txt res/d.txt res/e.txt res/f.txt -o out/a.out out/b.out out/c.out out/d.out out/e.out out/f.out` | 31.345 ± 0.039 | 31.317 | 31.372 | 1.00 |

## Check before doing expensive computation (f8d97b4)

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/ghc2021-score res/a.txt res/b.txt res/c.txt res/d.txt res/e.txt res/f.txt -o out/a.out out/b.out out/c.out out/d.out out/e.out out/f.out` | 10.312 ± 0.078 | 10.252 | 10.441 | 1.00 |

## Use IndexMap (0fd470c)

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/ghc2021-score res/a.txt res/b.txt res/c.txt res/d.txt res/e.txt res/f.txt -o out/a.out out/b.out out/c.out out/d.out out/e.out out/f.out` | 7.418 ± 0.066 | 7.348 | 7.483 | 1.00 |

## Remove empty queues (37557cb)

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/ghc2021-score res/a.txt res/b.txt res/c.txt res/d.txt res/e.txt res/f.txt -o out/a.out out/b.out out/c.out out/d.out out/e.out out/f.out` | 1.762 ± 0.010 | 1.754 | 1.779 | 1.00 |

## Merge 2 IndexMap together (53aa4be)

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/ghc2021-score-single-indexmap res/a.txt res/b.txt res/c.txt res/d.txt res/e.txt res/f.txt -o out/a.out out/b.out out/c.out out/d.out out/e.out out/f.out` | 1.640 ± 0.026 | 1.602 | 1.677 | 1.00 |