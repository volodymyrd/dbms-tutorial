use rand::prelude::*;
use std::time::Instant;

// Row-Oriented Layout (NSM)
#[repr(C)]
struct GenomicRow {
    id: u32,
    pos: u32,
    kmer: u32,
}

type GenomicRowStore = Vec<GenomicRow>;

// Column-Oriented Layout (DSM)
#[allow(dead_code)]
struct GenomicColStore {
    ids: Vec<u32>,
    pos: Vec<u32>,
    kmers: Vec<u32>,
}

fn create_row_store(n: usize) -> GenomicRowStore {
    let mut rng = rand::rng();
    let mut store = Vec::with_capacity(n);
    for _ in 0..n {
        store.push(GenomicRow {
            id: rng.random(),
            pos: rng.random(),
            kmer: rng.random(),
        });
    }
    store
}
fn create_col_store(n: usize) -> GenomicColStore {
    let mut rng = rand::rng();
    let mut ids = vec![0; n];
    let mut pos = vec![0; n];
    let mut kmers = vec![0; n];

    for i in 0..n {
        ids[i] = rng.random();
        pos[i] = rng.random();
        kmers[i] = rng.random();
    }
    GenomicColStore { ids, pos, kmers }
}

fn sum_pos_in_row_store(store: &GenomicRowStore) -> u128 {
    store.iter().map(|s| s.pos as u128).sum()
}

fn sum_pos_in_col_store(store: &GenomicColStore) -> u128 {
    store.pos.iter().map(|&e| e as u128).sum()
}

fn main() {
    let n = 10_000_000;
    let row_store = create_row_store(n);
    let col_store = create_col_store(n);
    println!("Stores are created");
    let start = Instant::now();
    let s = sum_pos_in_row_store(&row_store);
    let duration = start.elapsed();
    println!(
        "Time elapsed in sum_pos_in_row_store is: {:?} with value {s}",
        duration
    );

    let start = Instant::now();
    let s = sum_pos_in_col_store(&col_store);
    let duration = start.elapsed();
    println!(
        "Time elapsed in sum_pos_in_col_store is: {:?} with value {s}",
        duration
    );
}
