use criterion::{black_box, criterion_group, criterion_main, Criterion};
use open_lark::core::query_params::QueryParams;
use std::collections::HashMap;

fn benchmark_old_approach(c: &mut Criterion) {
    c.bench_function("old_string_allocation", |b| {
        b.iter(|| {
            let mut params = HashMap::<String, String>::new();
            params.insert(
                black_box("page_size".to_string()),
                black_box("20".to_string()),
            );
            params.insert(
                black_box("page_token".to_string()),
                black_box("token_123".to_string()),
            );
            params.insert(
                black_box("user_id".to_string()),
                black_box("user_456".to_string()),
            );
            params.insert(
                black_box("start_time".to_string()),
                black_box("2024-01-01T00:00:00Z".to_string()),
            );
            params.insert(
                black_box("end_time".to_string()),
                black_box("2024-01-31T23:59:59Z".to_string()),
            );
            params
        })
    });
}

fn benchmark_new_approach(c: &mut Criterion) {
    c.bench_function("new_static_constants", |b| {
        b.iter(|| {
            let mut params = HashMap::<&'static str, String>::new();
            params.insert(
                black_box(QueryParams::PAGE_SIZE),
                black_box("20".to_string()),
            );
            params.insert(
                black_box(QueryParams::PAGE_TOKEN),
                black_box("token_123".to_string()),
            );
            params.insert(
                black_box(QueryParams::USER_ID),
                black_box("user_456".to_string()),
            );
            params.insert(
                black_box(QueryParams::START_TIME),
                black_box("2024-01-01T00:00:00Z".to_string()),
            );
            params.insert(
                black_box(QueryParams::END_TIME),
                black_box("2024-01-31T23:59:59Z".to_string()),
            );
            params
        })
    });
}

fn benchmark_query_builder(c: &mut Criterion) {
    c.bench_function("query_builder_approach", |b| {
        b.iter(|| {
            open_lark::core::query_params::QueryParamsBuilder::new()
                .page_size(black_box(20))
                .page_token(black_box("token_123"))
                .user_id(black_box("user_456"))
                .start_time(black_box("2024-01-01T00:00:00Z"))
                .end_time(black_box("2024-01-31T23:59:59Z"))
                .build()
        })
    });
}

criterion_group!(
    benches,
    benchmark_old_approach,
    benchmark_new_approach,
    benchmark_query_builder
);
criterion_main!(benches);
