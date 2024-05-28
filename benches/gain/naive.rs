use std::sync::Arc;

use autd3_driver::{defined::PI, firmware::cpu::TxDatagram, geometry::Vector3};

use autd3_gain_holo::{Naive, NalgebraBackend, Pa};
use criterion::{black_box, Criterion};

use crate::{generate_geometry, pack, WallTimeUs};

pub fn naive(c: &mut Criterion<WallTimeUs>) {
    let mut group = c.benchmark_group("autd3/gain");

    [(1, 2), (10, 2), (1, 10), (10, 10)]
        .iter()
        .for_each(|&(size, n)| {
            group.bench_with_input(
                format!("Gain::Naive/{}/{}", size, n),
                &(
                    generate_geometry(size),
                    Arc::new(NalgebraBackend::default()),
                ),
                |b, (geometry, backend)| {
                    let mut tx = TxDatagram::new(size);
                    b.iter(|| {
                        let g = Naive::new(backend.clone()).add_foci_from_iter((0..n).map(|i| {
                            (
                                Vector3::new(
                                    black_box(90. + 10. * (2.0 * PI * i as f64 / n as f64).cos()),
                                    black_box(70. + 10. * (2.0 * PI * i as f64 / n as f64).sin()),
                                    black_box(150.),
                                ),
                                5e3 * Pa * size as f64 / n as f64,
                            )
                        }));
                        pack(g, geometry, &mut tx);
                    })
                },
            );
        });
    group.finish();
}
