use criterion::{Criterion, criterion_group, criterion_main};
use parser::parse;

fn bench_parse(input: &str) -> Vec<hir::Stmt> {
    let parse = parse(input);
    let syntax = parse.syntax();
    let errors = ast::validation::validate(&syntax);
    assert!(errors.is_empty());

    let root = ast::Root::cast(syntax).unwrap();
    let (_database, stmts) = hir::lower(&root);
    assert!(!stmts.is_empty());

    stmts
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");

    // Configure Criterion.rs to detect smaller differences and increase sample
    // size to improve precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(1000);
    group.bench_function("literal", |b| b.iter(|| bench_parse("999")));
    group.bench_function("binary_expression", |b| {
        b.iter(|| bench_parse("1+2*3-(4/5)"))
    });
    group.bench_function("variable_definition", |b| {
        b.iter(|| bench_parse("let foo = 10"))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
