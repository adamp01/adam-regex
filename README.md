# Adam Regex

This project is intended as a learning resource for Rust, finite automata, and language interoperability. The goal is to build a working regex engine in Rust that can parse a string pattern and construct a deterministic finite automaton (DFA), which can then be used to match against user-supplied strings.

The engine will be exposed to Python via FFI, where it will be benchmarked against Python's built-in `re` module. Future work will likely include performance optimization, with an eye toward leveraging SIMD where applicable.

### Regex Benchmark: Matching on 10,000 character input

| Pattern     | Engine          | Time (µs) | Notes                        |
|-------------|------------------|-----------|------------------------------|
| `(a\|b)*`    | adam (v1)        | 210       | Char based, no DFA minimisation    |
| `(a\|b)*`    | adam (v2)        | 25       | u8 based, no DFA minimisation    |
| `(a\|b)*`    | std::regex       | 0.012     |   |
