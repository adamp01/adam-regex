use adam_regex::matcher::AdamRegex;
use criterion::{criterion_group, criterion_main, Criterion};
use regex::Regex as StdRegex;
use std::hint::black_box;

fn bench_simple_repetition(c: &mut Criterion) {
    let pattern = "(a|b)*";
    let input = "abababababababababababababababababab";

    let adam = AdamRegex::from_str(pattern).unwrap();
    let std = StdRegex::new(pattern).unwrap();

    c.bench_function("simple repetition - adam", |b| {
        b.iter(|| adam.matches(black_box(input)))
    });
    c.bench_function("simple repetition - regex", |b| {
        b.iter(|| std.is_match(black_box(input)))
    });
}

fn bench_nested_star(c: &mut Criterion) {
    let pattern = "((a*)*)*";
    let input = "a".repeat(1000);

    let adam = AdamRegex::from_str(pattern).unwrap();
    let std = StdRegex::new(pattern).unwrap();

    c.bench_function("nested star - adam", |b| {
        b.iter(|| adam.matches(black_box(&input)))
    });
    c.bench_function("nested star - regex", |b| {
        b.iter(|| std.is_match(black_box(&input)))
    });
}

fn bench_alt_explosion(c: &mut Criterion) {
    let pattern = "(ab|cd|ef|gh|ij|kl|mn|op)*z";
    let input = "abefcdghmnklijop".repeat(1000) + "z";

    let adam = AdamRegex::from_str(pattern).unwrap();
    let std = StdRegex::new(pattern).unwrap();

    c.bench_function("alt explosion - adam", |b| {
        b.iter(|| adam.matches(black_box(&input)))
    });
    c.bench_function("alt explosion - regex", |b| {
        b.iter(|| std.is_match(black_box(&input)))
    });
}

fn bench_long_concat(c: &mut Criterion) {
    let pattern = "a".repeat(100);
    let input = "a".repeat(100);

    let adam = AdamRegex::from_str(&pattern).unwrap();
    let std = StdRegex::new(&pattern).unwrap();

    c.bench_function("long concat - adam", |b| {
        b.iter(|| adam.matches(black_box(&input)))
    });
    c.bench_function("long concat - regex", |b| {
        b.iter(|| std.is_match(black_box(&input)))
    });
}

fn bench_suffix_fail(c: &mut Criterion) {
    let pattern = "(a|b)*z";
    let input = "a".repeat(10000);

    let adam = AdamRegex::from_str(pattern).unwrap();
    let std = StdRegex::new(pattern).unwrap();

    c.bench_function("suffix fail - adam", |b| {
        b.iter(|| adam.matches(black_box(&input)))
    });
    c.bench_function("suffix fail - regex", |b| {
        b.iter(|| std.is_match(black_box(&input)))
    });
}

fn bench_dot_wildcard(c: &mut Criterion) {
    let pattern = ".".repeat(1000);
    let input = "a".repeat(1000);

    let adam = AdamRegex::from_str(&pattern).unwrap();
    let std = StdRegex::new(&pattern).unwrap();

    c.bench_function("dot wildcard - adam", |b| {
        b.iter(|| adam.matches(black_box(&input)))
    });
    c.bench_function("dot wildcard - regex", |b| {
        b.iter(|| std.is_match(black_box(&input)))
    });
}

fn bench_plus_repetition(c: &mut Criterion) {
    let pattern = "a+";
    let input = "a".repeat(10000);

    let adam = AdamRegex::from_str(pattern).unwrap();
    let std = StdRegex::new(pattern).unwrap();

    c.bench_function("plus repetition - adam", |b| {
        b.iter(|| adam.matches(black_box(&input)))
    });
    c.bench_function("plus repetition - regex", |b| {
        b.iter(|| std.is_match(black_box(&input)))
    });
}

fn bench_optional_char(c: &mut Criterion) {
    let pattern = "a?".repeat(1000);
    let input = "a".repeat(500);

    let adam = AdamRegex::from_str(&pattern).unwrap();
    let std = StdRegex::new(&pattern).unwrap();

    c.bench_function("optional char - adam", |b| {
        b.iter(|| adam.matches(black_box(&input)))
    });
    c.bench_function("optional char - regex", |b| {
        b.iter(|| std.is_match(black_box(&input)))
    });
}

fn bench_mixed_dot_plus_optional(c: &mut Criterion) {
    let pattern = "a?.b+";
    let input = String::from("az") + &"b".repeat(1000);

    let adam = AdamRegex::from_str(pattern).unwrap();
    let std = StdRegex::new(pattern).unwrap();

    c.bench_function("mixed dot plus opt - adam", |b| {
        b.iter(|| adam.matches(black_box(&input)))
    });
    c.bench_function("mixed dot plus opt - regex", |b| {
        b.iter(|| std.is_match(black_box(&input)))
    });
}

criterion_group!(
    benches,
    bench_simple_repetition,
    bench_nested_star,
    bench_alt_explosion,
    bench_long_concat,
    bench_suffix_fail,
    bench_dot_wildcard,
    bench_plus_repetition,
    bench_optional_char,
    bench_mixed_dot_plus_optional,
);
criterion_main!(benches);
