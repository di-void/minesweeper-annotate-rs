mod annotate_one;
mod annotate_two;

use annotate_one::annotate as original_annotate;
use criterion::{criterion_group, criterion_main, Criterion};

use annotate_two::annotate as simplified_annotate;

fn benchmark_annotate(c: &mut Criterion) {
    let minefield = vec!["*  *  ", "  *   ", "    * ", "*     ", "  *  *", " *  * "];

    c.bench_function("original annotate", |b| {
        b.iter(|| original_annotate(&minefield))
    });
    c.bench_function("simplified annotate", |b| {
        b.iter(|| simplified_annotate(&minefield))
    });
}

criterion_group!(benches, benchmark_annotate);
criterion_main!(benches);
