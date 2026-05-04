use rand::{self, RngExt};
use std::{fs::File, io::Write};

#[repr(C)]
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
    let byte_count = genomic_row_store.len() * std::mem::size_of::<GenomicRow>();

    // SAFETY:
    // 1. GenomicRow is #[repr(C)] and contains only primitive u32s.
    //    It has no internal padding, pointers, or uninitialized memory.
    // 2. `genomic_row_store.as_ptr()` points to a valid, contiguous heap allocation.
    // 3. `byte_count` exactly matches the length of the allocation.
    // 4. We only read from this slice during the `write_all` call; we do not mutate it.
    let byte_slice: &[u8] =
        unsafe { std::slice::from_raw_parts(genomic_row_store.as_ptr() as *const u8, byte_count) };

    file.write_all(byte_slice).expect("Failed to write to disk");
    println!("Done! File 'immutable_data.bin' created successfully.");
}
