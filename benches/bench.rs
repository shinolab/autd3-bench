mod gain;
mod modulation;
mod stm;

use std::time::{Duration, Instant};

use autd3::{derive::Datagram, prelude::*};
use autd3_driver::{
    defined::FREQ_40K,
    firmware::{cpu::TxDatagram, operation::OperationHandler},
    geometry::IntoDevice,
};

use gain::*;
use modulation::*;
use stm::*;

use criterion::{
    criterion_group, criterion_main,
    measurement::{Measurement, ValueFormatter},
    Criterion, Throughput,
};

pub fn generate_geometry(size: usize) -> Geometry {
    Geometry::new(
        (0..size)
            .map(move |i| {
                AUTD3::new(Vector3::new(i as f64 * AUTD3::DEVICE_WIDTH, 0., 0.)).into_device(i)
            })
            .collect(),
        FREQ_40K,
    )
}

fn pack<'a>(d: impl Datagram<'a>, geometry: &Geometry, tx: &mut TxDatagram) {
    let parallel_threshold = d.parallel_threshold().unwrap_or(4);

    let gen = d.operation_generator(geometry).unwrap();
    let mut operations = OperationHandler::generate(gen, geometry).unwrap();
    loop {
        OperationHandler::pack(&mut operations, geometry, tx, parallel_threshold).unwrap();
        if OperationHandler::is_done(&operations, geometry) {
            break;
        }
    }
}

pub(crate) struct UsDurationFormatter;
impl UsDurationFormatter {
    fn bytes_per_second(&self, bytes: f64, typical: f64, values: &mut [f64]) -> &'static str {
        let bytes_per_second = bytes * (1e9 / typical);
        let (denominator, unit) = if bytes_per_second < 1024.0 {
            (1.0, "  B/s")
        } else if bytes_per_second < 1024.0 * 1024.0 {
            (1024.0, "KiB/s")
        } else if bytes_per_second < 1024.0 * 1024.0 * 1024.0 {
            (1024.0 * 1024.0, "MiB/s")
        } else {
            (1024.0 * 1024.0 * 1024.0, "GiB/s")
        };

        for val in values {
            let bytes_per_second = bytes * (1e9 / *val);
            *val = bytes_per_second / denominator;
        }

        unit
    }

    fn bytes_per_second_decimal(
        &self,
        bytes: f64,
        typical: f64,
        values: &mut [f64],
    ) -> &'static str {
        let bytes_per_second = bytes * (1e9 / typical);
        let (denominator, unit) = if bytes_per_second < 1000.0 {
            (1.0, "  B/s")
        } else if bytes_per_second < 1000.0 * 1000.0 {
            (1000.0, "KB/s")
        } else if bytes_per_second < 1000.0 * 1000.0 * 1000.0 {
            (1000.0 * 1000.0, "MB/s")
        } else {
            (1000.0 * 1000.0 * 1000.0, "GB/s")
        };

        for val in values {
            let bytes_per_second = bytes * (1e9 / *val);
            *val = bytes_per_second / denominator;
        }

        unit
    }

    fn elements_per_second(&self, elems: f64, typical: f64, values: &mut [f64]) -> &'static str {
        let elems_per_second = elems * (1e9 / typical);
        let (denominator, unit) = if elems_per_second < 1000.0 {
            (1.0, " elem/s")
        } else if elems_per_second < 1000.0 * 1000.0 {
            (1000.0, "Kelem/s")
        } else if elems_per_second < 1000.0 * 1000.0 * 1000.0 {
            (1000.0 * 1000.0, "Melem/s")
        } else {
            (1000.0 * 1000.0 * 1000.0, "Gelem/s")
        };

        for val in values {
            let elems_per_second = elems * (1e9 / *val);
            *val = elems_per_second / denominator;
        }

        unit
    }
}

impl ValueFormatter for UsDurationFormatter {
    fn scale_throughputs(
        &self,
        typical: f64,
        throughput: &Throughput,
        values: &mut [f64],
    ) -> &'static str {
        match *throughput {
            Throughput::Bytes(bytes) => self.bytes_per_second(bytes as f64, typical, values),
            Throughput::BytesDecimal(bytes) => {
                self.bytes_per_second_decimal(bytes as f64, typical, values)
            }
            Throughput::Elements(elems) => self.elements_per_second(elems as f64, typical, values),
        }
    }

    fn scale_values(&self, _ns: f64, values: &mut [f64]) -> &'static str {
        let (factor, unit) = (10f64.powi(-3), "µs");

        for val in values {
            *val *= factor;
        }

        unit
    }

    fn scale_for_machines(&self, _values: &mut [f64]) -> &'static str {
        "ns"
    }
}

pub struct WallTimeUs;
impl Measurement for WallTimeUs {
    type Intermediate = Instant;
    type Value = Duration;

    fn start(&self) -> Self::Intermediate {
        Instant::now()
    }
    fn end(&self, i: Self::Intermediate) -> Self::Value {
        i.elapsed()
    }
    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        *v1 + *v2
    }
    fn zero(&self) -> Self::Value {
        Duration::from_secs(0)
    }
    fn to_f64(&self, val: &Self::Value) -> f64 {
        val.as_nanos() as f64
    }
    fn formatter(&self) -> &dyn ValueFormatter {
        &UsDurationFormatter
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().without_plots().with_measurement(WallTimeUs{});
    targets = focus, greedy, naive, sine, gain_stm, focus_stm);
criterion_main!(benches);
