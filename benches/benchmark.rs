use criterion::{Criterion, criterion_group, criterion_main};
use get_score::generate_game;
use rand::Rng;
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let game = generate_game();

    // We select, except for the first one, since it is the initiator and has zero values.
    let score_idx = rand::rng().random_range(1..game.len());
    let expected_stamp = game[score_idx];

    let mut group = c.benchmark_group("get_score");

    group.bench_function("lite_with_default", |b| {
        b.iter(|| get_score::lite::with_default::get_score(&game, black_box(expected_stamp.offset)))
    });

    group.bench_function("lite_with_panic", |b| {
        b.iter(|| get_score::lite::with_panic::get_score(&game, black_box(expected_stamp.offset)))
    });

    group.bench_function("best_with_default", |b| {
        b.iter(|| get_score::best::with_default::get_score(&game, black_box(expected_stamp.offset)))
    });

    group.bench_function("best_with_panic", |b| {
        b.iter(|| get_score::best::with_panic::get_score(&game, black_box(expected_stamp.offset)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
