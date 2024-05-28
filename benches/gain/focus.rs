use autd3::{gain::Focus};
use autd3_driver::{
    firmware::{cpu::TxDatagram},
    geometry::Vector3,
};

use criterion::{black_box, BenchmarkId, Criterion};

use crate::{generate_geometry, pack, WallTimeUs};

pub fn focus(c: &mut Criterion<WallTimeUs>) {
    let mut group = c.benchmark_group("autd3/gain");

    [1, 10].iter().for_each(|&size| {
        group.bench_with_input(
            BenchmarkId::new("Gain::Focus", size),
            &generate_geometry(size),
            |b, geometry| {
                let mut tx = TxDatagram::new(size);
                b.iter(|| {
                    let g = Focus::new(Vector3::new(
                        black_box(90.),
                        black_box(70.),
                        black_box(150.),
                    ));
                    pack(g, geometry, &mut tx);
                })
            },
        );
    });
    group.finish();
}
