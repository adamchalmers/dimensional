use criterion::{Criterion, criterion_group, criterion_main};
use dimensional::Dimensional;
use std::hint::black_box;

pub fn addition(c: &mut Criterion) {
    c.bench_function("adding lengths", |b| {
        b.iter(|| {
            // Put your code to benchmark here
            let a = Dimensional::feet(2.0);
            let b = Dimensional::cm(4.0);
            let c = Dimensional::mm(3.0);
            let _sum = black_box(a + b + c);
        })
    });
}
pub fn multiplication(c: &mut Criterion) {
    c.bench_function("multiplying lengths", |b| {
        b.iter(|| {
            // Put your code to benchmark here
            let a = Dimensional::feet(2.0);
            let b = Dimensional::cm(4.0);
            let _prod = black_box(a * b);
        })
    });
}

// Make a new benchmark suite called 'benches',
// It has two individual benchmarks: 'addition' and 'multiplication'
criterion_group!(benches, addition, multiplication);
// Generate a basic Rust binary that sets up Criterion
// and calls 'benches'.
criterion_main!(benches);
