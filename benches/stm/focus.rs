use autd3_driver::{datagram::FocusSTM, defined::PI, firmware::cpu::TxDatagram, geometry::Vector3};

use criterion::{black_box, Criterion};

use crate::{generate_geometry, pack, WallTimeUs};

pub fn focus_stm(c: &mut Criterion<WallTimeUs>) {
    let mut group = c.benchmark_group("autd3/stm");

    [
        (1, 2),
        (10, 2),
        (1, 5000),
        (10, 5000),
        (1, 65536),
        (10, 65536),
    ]
    .iter()
    .for_each(|&(size, n)| {
        group.bench_with_input(
            format!("STM::Focus/{}/{}", size, n),
            &generate_geometry(size),
            |b, geometry| {
                let mut tx = TxDatagram::new(size);
                b.iter(|| {
                    let g = FocusSTM::from_sampling_config(
                        autd3::derive::SamplingConfig::DivisionRaw(512),
                        (0..n).map(|i| {
                            Vector3::new(
                                black_box(90. + 10. * (2.0 * PI * i as f64 / n as f64).cos()),
                                black_box(70. + 10. * (2.0 * PI * i as f64 / n as f64).sin()),
                                black_box(150.),
                            )
                        }),
                    );
                    pack(g, geometry, &mut tx);
                })
            },
        );
    });
    group.finish();
}
