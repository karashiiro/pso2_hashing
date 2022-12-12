use self::models::*;
use diesel::prelude::*;
use pso2_rainbow::*;

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
}
