use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;
use tempfile;

fn bench_insertion_profile(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-insertion-profile");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bam = manifest.join("tests/golden/ins.bam");
    let dir = tempfile::tempdir().unwrap();
    let prefix = dir.path().join("out");
    c.bench_function("rsomics-insertion-profile golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .args([
                    "-i",
                    bam.to_str().unwrap(),
                    "-o",
                    prefix.to_str().unwrap(),
                ])
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_insertion_profile);
criterion_main!(benches);
