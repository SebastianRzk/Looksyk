use criterion::{criterion_group, criterion_main, Criterion};
use looksyk::io::fs::pages::PageOnDisk;
use looksyk::looksyk::index::userpage::create_user_page_index;

fn bench_parse_text_content(c: &mut Criterion) {
    let mut group = c.benchmark_group("Text Parsing");
    let input_short = include_str!("../tests/test_input_short.txt");
    let short_pages = &[PageOnDisk {
        name: "test".to_string(),
        content: input_short.to_string(),
    }];

    let input_medium = include_str!("../tests/test_input_medium.txt");
    let medium_pages = &[PageOnDisk {
        name: "test_medium".to_string(),
        content: input_medium.to_string(),
    }];

    let input_long = include_str!("../tests/test_input_long.txt");
    let long_pages = &[PageOnDisk {
        name: "test_long".to_string(),
        content: input_long.to_string(),
    }];

    group.bench_function("parse_text_content_short", |b| {
        b.iter(|| create_user_page_index(short_pages))
    });
    group.bench_function("parse_text_content_medium", |b| {
        b.iter(|| create_user_page_index(medium_pages))
    });
    group.bench_function("parse_text_content_long", |b| {
        b.iter(|| create_user_page_index(long_pages))
    });
    group.finish();
}

criterion_group!(benches, bench_parse_text_content);
criterion_main!(benches);
