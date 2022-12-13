pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use models::NewHashMapping;
use std::env;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("could not build connection pool")
}

pub fn create_hash_mappings(conn: &mut PgConnection, hashes: &[NewHashMapping]) {
    use crate::schema::hash_mapping;

    diesel::insert_into(hash_mapping::table)
        .values(hashes)
        .on_conflict_do_nothing()
        .execute(conn)
        .unwrap_or_else(|e| {
            process_error(e);
            0
        });
}

fn process_error(e: diesel::result::Error) {
    println!("{:?}", e)
}
