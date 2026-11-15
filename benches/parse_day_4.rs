use advent_of_code::day4::{
    count_letters, count_letters_arr, parse_checksum, parse_line, ranked_letters, top_five,
};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

const LINE: &str = "aaaaa-bbb-z-y-x-123[abxyz]";

fn stages(c: &mut Criterion) {
    // setup: outputs of earlier stages become inputs of later ones
    let (encoded, _, _) = parse_line(LINE);
    let counts = count_letters(encoded);
    let counts_arr = count_letters_arr(encoded);

    let mut g = c.benchmark_group("stages");
    g.bench_function("parse_line", |b| b.iter(|| parse_line(black_box(LINE))));
    g.bench_function("count_letters", |b| {
        b.iter(|| count_letters(black_box(encoded)))
    });
    g.bench_function("count_letters_arr", |b| {
        b.iter(|| count_letters_arr(black_box(encoded)))
    });
    g.bench_function("top_five", |b| {
        b.iter(|| top_five(black_box(counts.clone())))
    });
    g.bench_function("top", |b| b.iter(|| ranked_letters(black_box(counts_arr))));
    g.bench_function("whole", |b| b.iter(|| parse_checksum(black_box(LINE))));
    g.finish();
}
criterion_group!(benches, stages);
criterion_main!(benches);
