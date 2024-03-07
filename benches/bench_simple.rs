use criterion::{criterion_group, criterion_main, Criterion};
use c_emit::{CArg, Code};

fn bench_simple(c: &mut Criterion) {
    c.bench_function("bench_simple", |b| {
        b.iter(|| {
            let mut c = Code::new();

            c.exit(1);
            c.call_func_with_args("printf", vec![CArg::String("Hello World!".to_string())]);
            c.call_func("printf");
            c.include("stdio.h");

            println!("{c}");
        });
    });
}

criterion_group!(benches, bench_simple);
criterion_main!(benches);