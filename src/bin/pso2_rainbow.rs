use self::models::*;
use diesel::prelude::*;
use generic_array::typenum::U16;
use generic_array::GenericArray;
use itertools::Itertools;
use md5::{Digest, Md5};
use pso2_rainbow::*;
use rayon::prelude::*;

const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz0123456789/_.";

fn hash_chars(chars: Vec<char>) -> GenericArray<u8, U16> {
    let string = String::from_iter(chars);
    let mut hasher = Md5::new();
    hasher.update(string);
    hasher.finalize()
}

fn main() {
    use self::schema::hash_mapping::dsl::*;

    let connection = &mut establish_connection();
    let results = hash_mapping
        .load::<HashMapping>(connection)
        .expect("Error loading hashes");

    println!("Displaying {} hashes", results.len());
    for hash in results {
        println!("{:?}:{}", hash.md5, hash.filename);
    }

    let min_length = 4;
    let max_length = 8;
    let suffix = ".ice".chars().collect_vec();
    (min_length - suffix.len()..max_length - suffix.len() + 1)
        .flat_map(|n| CHARSET.chars().permutations(n))
        .par_bridge()
        .map(|chars| [chars, suffix.clone()].concat())
        .map(hash_chars)
        .for_each(|hash| println!("{}", hex::encode(hash)));
}
