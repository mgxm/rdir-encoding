#[macro_use]
extern crate criterion;
extern crate rdir_encoding;

use criterion::Criterion;
use rdir_encoding::{Delta, IntegerEncoding, RecursiveIndexing, RunLength};

fn run_lenght_decode() {
    let encoded = [1, 4, 2, 1, 1, 4];
    RunLength::decode(&encoded).unwrap();
}

fn run_lenght_encode() {
    let encoded = [1, 1, 1, 1, 2, 1, 1, 1, 1];
    RunLength::encode(&encoded).unwrap();
}

fn delta_decode() {
    let data = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 5];
    Delta::decode(&data).unwrap();
}

fn delta_encode() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 20];
    Delta::encode(&data).unwrap();
}

fn integer_decode() {
    let data = [100, 100, 100, 100, 50, 50];
    IntegerEncoding::decode(&data, 100).unwrap();
}

fn integer_encode() {
    let data = [1.00, 1.00, 1.00, 1.00, 0.50, 0.50];
    IntegerEncoding::encode(&data, 100).unwrap();
}

fn recursive_index_decode() {
    let data = [1, 420, 32767, 0, 120, -32768, 0, 32767, 2];
    RecursiveIndexing::decode(&data).unwrap();
}

fn recursive_index_encode() {
    let data = [1, 420, 32767, 120, -32768, 32769];
    RecursiveIndexing::encode(&data).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Run Lenght Decode", |b| b.iter(|| run_lenght_decode()));
    c.bench_function("Run Lenght Encode", |b| b.iter(|| run_lenght_encode()));
    c.bench_function("Delta Decode", |b| b.iter(|| delta_decode()));
    c.bench_function("Delta Encode", |b| b.iter(|| delta_encode()));
    c.bench_function("Integer Decode", |b| b.iter(|| integer_decode()));
    c.bench_function("Integer Encode", |b| b.iter(|| integer_encode()));
    c.bench_function("Recursive Index Decode", |b| {
        b.iter(|| recursive_index_decode())
    });
    c.bench_function("Recursive Index Encode", |b| {
        b.iter(|| recursive_index_encode())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
