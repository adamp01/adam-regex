use adam_regex::matcher::AdamRegex;
use criterion::{criterion_group, criterion_main, Criterion};
use regex::Regex as StdRegex;
use std::hint::black_box;

fn bench_adam_regex(c: &mut Criterion) {
    let input = "a".repeat(10_000);
    let pattern = "(a|b)*";
    let re = AdamRegex::from_str(pattern).unwrap();

    c.bench_function("adam_regex (a|b)* 10k", |b| {
        b.iter(|| {
            let result = re.matches(black_box(&input));
            black_box(result);
        });
    });
}

fn bench_std_regex(c: &mut Criterion) {
    let input = "a".repeat(10_000);
    let pattern = "(a|b)*";
    let re = StdRegex::new(pattern).unwrap();

    c.bench_function("std::regex (a|b)* 10k", |b| {
        b.iter(|| {
            let result = re.is_match(black_box(&input));
            black_box(result);
        });
    });
}

criterion_group!(benches, bench_adam_regex, bench_std_regex);
criterion_main!(benches);
