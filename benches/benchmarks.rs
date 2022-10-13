use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    type TestType = Vec<u8>;

    c.bench_function("inline", |b| b.iter(|| include!("../inline.in")));
    c.bench_function("deserialize", |b| {
        b.iter(|| bincode::deserialize::<TestType>(include_bytes!("../vec.bin")).unwrap())
    });

    // Re-generates data
    // use rand::Rng;
    // use benching_heap_serial::Inline;
    // use std::io::Write;
    // let mut rng = rand::thread_rng();
    // let vec = (0..1000).map(|_| rng.gen()).collect::<TestType>();
    // let serialized = bincode::serialize(&vec).unwrap();
    // let mut file = std::fs::File::create("vec.bin").unwrap();
    // file.write_all(&serialized).unwrap();

    // let inline = vec.inline();
    // let mut file = std::fs::File::create("inline.in").unwrap();
    // file.write_all(inline.to_string().as_bytes()).unwrap();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
