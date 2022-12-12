use std::iter;

use self::models::*;
use diesel::prelude::*;
use itertools::Itertools;
use pso2_rainbow::*;

const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz0123456789/_.";

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

    let strings = (1..3).fold(
        Box::new(iter::empty::<String>()) as Box<dyn Iterator<Item = String>>,
        |it, n| {
            Box::new(it.chain(CHARSET.chars().permutations(n).unique().map(|v| {
                String::from_utf8(v.into_iter().map(|c| c as u8).collect_vec())
                    .expect("ASCII char tuples should always resolve to an owned string")
            })))
        },
    );
    for string in strings {
        println!("{}", string)
    }
}
