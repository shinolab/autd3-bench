use autd3::modulation::Sine;
use autd3_driver::{defined::Hz, firmware::cpu::TxDatagram};

use criterion::{black_box, BenchmarkId, Criterion};

use crate::{generate_geometry, pack, WallTimeUs};

pub fn sine(c: &mut Criterion<WallTimeUs>) {
    let mut group = c.benchmark_group("autd3/modulation");

    [1, 10].iter().for_each(|&size| {
        group.bench_with_input(
            BenchmarkId::new("Modulation::Sine", size),
            &generate_geometry(size),
            |b, geometry| {
                let mut tx = TxDatagram::new(size);
                b.iter(|| {
                    let g = Sine::new(black_box(150.) * Hz);
                    pack(g, geometry, &mut tx);
                })
            },
        );
    });
    group.finish();
}
