use crate::schema::hash_mapping;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct HashMapping {
    pub md5: Vec<u8>,
    pub filename: String,
}

#[derive(Insertable)]
#[diesel(table_name = hash_mapping)]
pub struct NewHashMapping<'a> {
    pub md5: &'a [u8],
    pub filename: &'a str,
}
