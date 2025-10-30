//! Sequential access benchmarks

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mmap_rs::{MemoryAdvice, MmapOptions};
use std::io::{Read, Write};
use tempfile::NamedTempFile;

fn create_test_file(size: usize) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    let data = vec![0xAB; size];
    file.write_all(&data).unwrap();
    file.flush().unwrap();
    file
}

fn sequential_mmap_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_mmap_read");

    for size in [1 << 16, 1 << 20, 1 << 24] {
        // 64KB, 1MB, 16MB
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new("mmap", size), &size, |b, &size| {
            let file = create_test_file(size);
            let mmap = MmapOptions::new().path(file.path()).map_readonly().unwrap();

            b.iter(|| {
                let mut sum: u64 = 0;
                for byte in mmap.as_slice() {
                    sum = sum.wrapping_add(*byte as u64);
                }
                black_box(sum)
            });
        });
    }

    group.finish();
}

fn sequential_mmap_read_with_advice(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_mmap_read_with_advice");

    for size in [1 << 16, 1 << 20, 1 << 24] {
        // 64KB, 1MB, 16MB
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::new("mmap_sequential", size),
            &size,
            |b, &size| {
                let file = create_test_file(size);
                let mmap = MmapOptions::new()
                    .path(file.path())
                    .advice(MemoryAdvice::Sequential)
                    .map_readonly()
                    .unwrap();

                b.iter(|| {
                    let mut sum: u64 = 0;
                    for byte in mmap.as_slice() {
                        sum = sum.wrapping_add(*byte as u64);
                    }
                    black_box(sum)
                });
            },
        );
    }

    group.finish();
}

fn sequential_file_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_file_read");

    for size in [1 << 16, 1 << 20, 1 << 24] {
        // 64KB, 1MB, 16MB
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new("std_read", size), &size, |b, &size| {
            let file = create_test_file(size);

            b.iter(|| {
                let mut file = std::fs::File::open(file.path()).unwrap();
                let mut buffer = vec![0u8; size];
                file.read_exact(&mut buffer).unwrap();

                let mut sum: u64 = 0;
                for byte in &buffer {
                    sum = sum.wrapping_add(*byte as u64);
                }
                black_box(sum)
            });
        });
    }

    group.finish();
}

fn sequential_mmap_read_prefault(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_mmap_read_prefault");

    for size in [1 << 16, 1 << 20, 1 << 24] {
        // 64KB, 1MB, 16MB
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::new("mmap_prefault", size),
            &size,
            |b, &size| {
                let file = create_test_file(size);
                let mmap = MmapOptions::new()
                    .path(file.path())
                    .populate()
                    .map_readonly()
                    .unwrap();

                b.iter(|| {
                    let mut sum: u64 = 0;
                    for byte in mmap.as_slice() {
                        sum = sum.wrapping_add(*byte as u64);
                    }
                    black_box(sum)
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    sequential_mmap_read,
    sequential_mmap_read_with_advice,
    sequential_file_read,
    sequential_mmap_read_prefault
);
criterion_main!(benches);
