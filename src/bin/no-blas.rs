extern crate blas_src;
use ndarray::Array2;
use pprof::protos::Message;
use std::fs::File;
use std::io::Write;

fn main() {
    let guard = pprof::ProfilerGuard::new(1000000).unwrap();
    for _i in 0..20 {
        let a = Array2::<f64>::zeros((10000, 1000));
        let b = Array2::<f64>::zeros((10000, 1000));
        let _ = a.t().dot(&b);
    }

    if let Ok(report) = guard.report().build() {
        let mut file = File::create("no-blas.pb").unwrap();
        let profile = report.pprof().unwrap();
        let mut content = Vec::new();
        profile.encode(&mut content).unwrap();
        file.write_all(&content).unwrap();
    }
}
