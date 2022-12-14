use generic_array::typenum::U16;
use generic_array::GenericArray;
use itertools::Itertools;
use md5::{Digest, Md5};
use pso2_rainbow::{models::NewHashMapping, *};
use rayon::prelude::*;
use tokio::runtime::Builder;

const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz0123456789/_";

fn hash_string(string: &str) -> GenericArray<u8, U16> {
    let mut hasher = Md5::new();
    hasher.update(string.chars().map(|c| c as u8).collect_vec());
    hasher.finalize()
}

fn build_graphemes<F>(charset: &[char], grapheme_size: usize, filter: F) -> Vec<String>
where
    F: Fn(&str) -> bool,
{
    charset
        .into_iter()
        .permutations(grapheme_size)
        .filter_map(|chars| {
            let s = String::from_iter(chars);
            if filter(&s) {
                Some(s)
            } else {
                None
            }
        })
        .collect_vec()
}

fn main() {
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    let connection_pool = get_connection_pool();

    // Build valid graphemes to minimize set of generated strings
    let grapheme_length = 2;
    let graphemes = build_graphemes(&CHARSET.chars().collect_vec(), grapheme_length, |s| {
        !s.contains("/_") && !s.contains("_/")
    });

    // Known file prefixes and suffixes
    let prefixes = vec![
        String::from(""),
        String::from("it_"),
        String::from("sy_"),
        String::from("ui_"),
        String::from("actor/"),
        String::from("apc/"),
        String::from("character/"),
        String::from("character/motion"),
        String::from("enemy/"),
        String::from("interface/"),
        String::from("lobby_action/"),
        String::from("section_fence/"),
        String::from("set/"),
    ];
    let suffixes = vec![String::from(".ice"), String::from(".cpk")];

    // Build input strings
    let permuted_min_length = 0 / grapheme_length;
    let permuted_max_length = 6 / grapheme_length;

    println!(
        "Strings to generate: {}",
        prefixes.len()
            * suffixes.len()
            * CHARSET
                .len()
                .pow((permuted_max_length - permuted_min_length) as u32)
    );

    let plaintext_chunks = prefixes
        .into_iter()
        .cartesian_product(
            (permuted_min_length..permuted_max_length + 1)
                .flat_map(|n| graphemes.clone().into_iter().permutations(n))
                .cartesian_product(suffixes.into_iter()),
        )
        .chunks(100000);

    for chunk in &plaintext_chunks {
        // Hash the input strings in parallel
        let hashes = &mut Vec::with_capacity(100000);
        chunk
            .collect_vec()
            .par_iter()
            .map(|(prefix, (g, suffix))| {
                [vec![prefix.clone()], g.clone(), vec![suffix.clone()]]
                    .into_iter()
                    .concat()
                    .into_iter()
                    .reduce(|accum, item| accum + &item)
                    .expect("iterator should not be empty")
            })
            .map(|s| (hash_string(&s), s))
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
