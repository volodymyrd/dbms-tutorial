use rand::{self, RngExt};
use std::{fs::File, io::Write};
use zerocopy::{Immutable, IntoBytes};

#[repr(C)]
#[derive(Clone, Copy, Immutable, IntoBytes)]
struct GenomicRow {
    id: u32,
    pos: u32,
    kmer: u32,
}
type GenomicRowStore = Vec<GenomicRow>;

fn create_row_store(n: usize) -> GenomicRowStore {
    println!("Generating {} records in RAM...", n);
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
fn main() {
    let mut genomic_row_store = create_row_store(1_000_000);
    println!("Sorting records by position...");
    genomic_row_store.sort_by_key(|r| r.pos);

    println!("Writing to immutable_data.bin...");
    let mut file = File::create("immutable_data.bin").expect("Failed to create a file");
    file.write_all(genomic_row_store.as_bytes())
        .expect("Failed to write to disk");
    println!("Done! File 'immutable_data.bin' created successfully.");
}
