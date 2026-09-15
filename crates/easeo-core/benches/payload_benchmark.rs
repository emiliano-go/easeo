use criterion::{black_box, criterion_group, criterion_main, Criterion};
use easeo_core::*;

// NOTE: Comparison benchmark with seoslug (Python) is omitted — different language,
// different runtime. Cross-language perf comparison belongs in a separate benchmark suite
// using a common harness (e.g., hyperfine calling both CLI wrappers).

fn bench_build_seo_payload(c: &mut Criterion) {
    let config = SEOConfig {
        canonical_host: "example.com".to_string(),
        public_base_url: "https://example.com".to_string(),
        ..Default::default()
    };

    let entity = SEOEntity {
        entity_type: EntityType::Post,
        title: Some("Benchmark Post".to_string()),
        excerpt: Some("A benchmark post description for testing purposes.".to_string()),
        published_at: Some("2026-01-01".to_string()),
        author_name: Some("Author".to_string()),
        ..Default::default()
    };

    c.bench_function("build_seo_payload", |b| {
        b.iter(|| {
            build_seo_payload(black_box(&entity), black_box("/blog/benchmark"), black_box(&config)).unwrap()
        })
    });
}

fn bench_build_batch(c: &mut Criterion) {
    let config = SEOConfig {
        canonical_host: "example.com".to_string(),
        public_base_url: "https://example.com".to_string(),
        ..Default::default()
    };

    let entity = SEOEntity {
        entity_type: EntityType::Post,
        title: Some("Benchmark Post".to_string()),
        excerpt: Some("A benchmark post description for testing purposes.".to_string()),
        published_at: Some("2026-01-01".to_string()),
        author_name: Some("Author".to_string()),
        ..Default::default()
    };

    let mut group = c.benchmark_group("build_batch");
    for size in [10, 100, 1_000, 10_000, 100_000, 1_000_000] {
        group.bench_with_input(format!("{}_payloads", size), &size, |b, &size| {
            b.iter(|| {
                for i in 0..size {
                    let route = format!("/blog/post-{}", i);
                    build_seo_payload(black_box(&entity), black_box(&route), black_box(&config)).unwrap();
                }
            })
        });
    }
    group.finish();
}

fn bench_hash_payload(c: &mut Criterion) {
    let config = SEOConfig {
        canonical_host: "example.com".to_string(),
        public_base_url: "https://example.com".to_string(),
        ..Default::default()
    };

    let entity = SEOEntity {
        entity_type: EntityType::Post,
        title: Some("Benchmark Post".to_string()),
        excerpt: Some("A benchmark post description for testing purposes.".to_string()),
        ..Default::default()
    };

    let payload = build_seo_payload(&entity, "/blog/benchmark", &config).unwrap();

    c.bench_function("hash_payload", |b| {
        b.iter(|| hash_payload(black_box(&payload)))
    });
}

fn bench_render_html(c: &mut Criterion) {
    let config = SEOConfig {
        canonical_host: "example.com".to_string(),
        public_base_url: "https://example.com".to_string(),
        ..Default::default()
    };

    let entity = SEOEntity {
        entity_type: EntityType::Post,
        title: Some("Benchmark Post".to_string()),
        excerpt: Some("A benchmark post description for testing purposes.".to_string()),
        ..Default::default()
    };

    let payload = build_seo_payload(&entity, "/blog/benchmark", &config).unwrap();

    c.bench_function("render_html", |b| {
        b.iter(|| payload.render_html())
    });
}

fn bench_validate(c: &mut Criterion) {
    let config = SEOConfig {
        canonical_host: "example.com".to_string(),
        public_base_url: "https://example.com".to_string(),
        ..Default::default()
    };

    let entity = SEOEntity {
        entity_type: EntityType::Post,
        title: Some("Benchmark Post".to_string()),
        excerpt: Some("A benchmark post description for testing purposes.".to_string()),
        ..Default::default()
    };

    let payload = build_seo_payload(&entity, "/blog/benchmark", &config).unwrap();

    c.bench_function("validate_payload", |b| {
        b.iter(|| validate_payload(black_box(&payload)))
    });
}

fn bench_contract(c: &mut Criterion) {
    let config = SEOContractConfig {
        canonical_host: "example.com".to_string(),
        scheme: "https".to_string(),
        ..Default::default()
    };

    c.bench_function("build_seo_contract", |b| {
        b.iter(|| build_seo_contract(black_box(&config)).unwrap())
    });
}

criterion_group!(
    benches,
    bench_build_seo_payload,
    bench_build_batch,
    bench_hash_payload,
    bench_render_html,
    bench_validate,
    bench_contract,
);
criterion_main!(benches);
