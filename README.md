# Google Hashcode 2021 Score Calculator

Computes Google Hashcode 2021 Qualification Round score.

It gives the same results as Google for our submissions.

It gives the sames result for the example file as well.

Caution: It may not be robust against incorrect submission file.

## Quickstart

example:

```
RUST_LOG=info cargo run --release -- res/a.txt res/b.txt res/c.txt res/d.txt res/e.txt res/f.txt -o out/a.out out/b.out out/c.out out/d.out out/e.out out/f.out
```

example output:

```
[2021-02-27T10:27:58Z INFO  ghc2021_score] parsing out/a.out
[2021-02-27T10:27:58Z INFO  ghc2021_score] parsing res/a.txt
out/a.out score: 2002
[2021-02-27T10:27:58Z INFO  ghc2021_score] parsing out/b.out
[2021-02-27T10:27:58Z INFO  ghc2021_score] parsing res/b.txt
out/b.out score: 4566783
[2021-02-27T10:27:59Z INFO  ghc2021_score] parsing out/c.out
[2021-02-27T10:27:59Z INFO  ghc2021_score] parsing res/c.txt
out/c.out score: 1299593
[2021-02-27T10:28:00Z INFO  ghc2021_score] parsing out/d.out
[2021-02-27T10:28:00Z INFO  ghc2021_score] parsing res/d.txt
out/d.out score: 1586428
[2021-02-27T10:28:17Z INFO  ghc2021_score] parsing out/e.out
[2021-02-27T10:28:17Z INFO  ghc2021_score] parsing res/e.txt
out/e.out score: 710095
[2021-02-27T10:28:17Z INFO  ghc2021_score] parsing out/f.out
[2021-02-27T10:28:17Z INFO  ghc2021_score] parsing res/f.txt
out/f.out score: 1408553
```

Note: it takes about **20s** on my desktop pc (`AMD Ryzen 7 3700X`)