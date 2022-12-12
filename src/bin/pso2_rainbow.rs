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

    let min_length = 4;
    let max_length = 5;
    let strings = (min_length..max_length + 1)
        .flat_map(|n| CHARSET.chars().permutations(n).map(String::from_iter));
    for string in strings {
        println!("{}", string)
    }
}
