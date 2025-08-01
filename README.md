# Adam Regex

This project is intended as a learning resource for Rust, finite automata, and language interoperability. The goal is to build a working regex engine in Rust that can parse a string pattern and construct a deterministic finite automaton (DFA), which can then be used to match against user-supplied strings.

The engine will be exposed to Python via FFI, where it will be benchmarked against Python's built-in `re` module. Future work will likely include performance optimization, with an eye toward leveraging SIMD where applicable.

### Regex Benchmark: Matching on 10,000 character input

| Pattern         | Engine        | Time (µs) | Notes |
|-----------------|---------------|-----------|-------|
| simple repetition | adam          |     0.051 |       |
| simple repetition | regex         |     0.013 |       |
| nested star     | adam          |     2.540 |       |
| nested star     | regex         |     0.014 |       |
| alt explosion   | adam          |    41.359 |       |
| alt explosion   | regex         |    69.750 |       |
| long concat     | adam          |     0.272 |       |
| long concat     | regex         |     0.071 |       |
| suffix fail     | adam          |    26.073 |       |
| suffix fail     | regex         |    17.161 |       |
| dot wildcard    | adam          |     0.002 |       |
| dot wildcard    | regex         |     2.072 |       |
| plus repetition | adam          |    25.809 |       |
| plus repetition | regex         |     0.017 |       |
| optional char   | adam          |     1.350 |       |
| optional char   | regex         |     0.013 |       |
| mixed dot plus opt | adam          |     0.002 |       |
| mixed dot plus opt | regex         |     0.042 |       |
