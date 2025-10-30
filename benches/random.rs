//! Random access benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use mmap_rs::{MemoryAdvice, MmapOptions};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::io::{Read, Seek, SeekFrom, Write};
use tempfile::NamedTempFile;

fn create_test_file(size: usize) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    let data = vec![0xAB; size];
    file.write_all(&data).unwrap();
    file.flush().unwrap();
    file
}

fn random_access_mmap(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_access_mmap");

    let size = 1 << 24; // 16MB
    let num_ops = 10000;

    group.throughput(Throughput::Elements(num_ops));
    group.bench_function("mmap_random", |b| {
        let file = create_test_file(size);
        let mmap = MmapOptions::new().path(file.path()).map_readonly().unwrap();

        let mut rng = StdRng::seed_from_u64(42);
        let offsets: Vec<usize> = (0..num_ops).map(|_| rng.gen_range(0..size)).collect();

        b.iter(|| {
            let mut sum: u64 = 0;
            for &offset in &offsets {
                sum = sum.wrapping_add(mmap[offset] as u64);
            }
            black_box(sum)
        });
    });

    group.finish();
}

fn random_access_mmap_with_advice(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_access_mmap_with_advice");

    let size = 1 << 24; // 16MB
    let num_ops = 10000;

    group.throughput(Throughput::Elements(num_ops));
    group.bench_function("mmap_random_advice", |b| {
        let file = create_test_file(size);
        let mmap = MmapOptions::new()
            .path(file.path())
            .advice(MemoryAdvice::Random)
            .map_readonly()
            .unwrap();

        let mut rng = StdRng::seed_from_u64(42);
        let offsets: Vec<usize> = (0..num_ops).map(|_| rng.gen_range(0..size)).collect();

        b.iter(|| {
            let mut sum: u64 = 0;
            for &offset in &offsets {
                sum = sum.wrapping_add(mmap[offset] as u64);
            }
            black_box(sum)
        });
    });

    group.finish();
}

fn random_access_file_seek(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_access_file_seek");

    let size = 1 << 24; // 16MB
    let num_ops = 10000;

    group.throughput(Throughput::Elements(num_ops));
    group.bench_function("std_seek_read", |b| {
        let file = create_test_file(size);

        let mut rng = StdRng::seed_from_u64(42);
        let offsets: Vec<u64> = (0..num_ops)
            .map(|_| rng.gen_range(0..size as u64))
            .collect();

        b.iter(|| {
            let mut file = std::fs::File::open(file.path()).unwrap();
            let mut sum: u64 = 0;
            let mut buffer = [0u8; 1];

            for &offset in &offsets {
                file.seek(SeekFrom::Start(offset)).unwrap();
                file.read_exact(&mut buffer).unwrap();
                sum = sum.wrapping_add(buffer[0] as u64);
            }
            black_box(sum)
        });
    });

    group.finish();
}

fn random_access_mmap_prefault(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_access_mmap_prefault");

    let size = 1 << 24; // 16MB
    let num_ops = 10000;

    group.throughput(Throughput::Elements(num_ops));
    group.bench_function("mmap_random_prefault", |b| {
        let file = create_test_file(size);
        let mmap = MmapOptions::new()
            .path(file.path())
            .populate()
            .map_readonly()
            .unwrap();

        let mut rng = StdRng::seed_from_u64(42);
        let offsets: Vec<usize> = (0..num_ops).map(|_| rng.gen_range(0..size)).collect();

        b.iter(|| {
            let mut sum: u64 = 0;
            for &offset in &offsets {
                sum = sum.wrapping_add(mmap[offset] as u64);
            }
            black_box(sum)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    random_access_mmap,
    random_access_mmap_with_advice,
    random_access_file_seek,
    random_access_mmap_prefault
);
criterion_main!(benches);
