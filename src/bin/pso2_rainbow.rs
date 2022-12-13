use generic_array::typenum::U16;
use generic_array::GenericArray;
use itertools::Itertools;
use md5::{Digest, Md5};
use pso2_rainbow::{models::NewHashMapping, *};
use rayon::prelude::*;
use tokio::runtime::Builder;

const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz0123456789/_.";

fn hash_chars(chars: Vec<char>) -> (GenericArray<u8, U16>, String) {
    let string = String::from_iter(chars);
    let mut hasher = Md5::new();
    hasher.update(string.clone());
    (hasher.finalize(), string)
}

fn main() {
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    let connection_pool = get_connection_pool();

    // Build input strings
    let min_length = 4;
    let max_length = 10;
    let suffix = ".ice".chars().collect_vec();
    let plaintext_chunks = (min_length - suffix.len()..max_length - suffix.len() + 1)
        .flat_map(|n| CHARSET.chars().permutations(n))
        .chunks(100000);
    for chunk in &plaintext_chunks {
        // Hash the input strings in parallel
        let hashes = &mut Vec::with_capacity(100000);
        chunk
            .collect_vec()
            .par_iter()
            .map(|chars| [chars.clone(), suffix.clone()].concat())
            .map(hash_chars)
            .collect_into_vec(hashes);

        // Batch-insert the hashes into the database
        let handles = &mut Vec::with_capacity(10);
        for batch in hashes.chunks(10000) {
            let batch = batch.to_owned();
            let mut connection = connection_pool
                .get()
                .expect("expected a connection from the connection pool");
            handles.push(runtime.spawn(async move {
                create_hash_mappings(
                    &mut connection,
                    &batch
                        .into_iter()
                        .map(|(hash, filename)| NewHashMapping {
                            md5: hash.to_vec(),
                            filename,
                        })
                        .collect_vec(),
                );
            }));
        }

        for handle in handles {
            runtime.block_on(handle).unwrap();
        }
    }
}
