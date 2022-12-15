use crate::schema::hash_mapping;
use diesel::{prelude::*};
use uuid::Uuid;

#[derive(Queryable)]
#[diesel(table_name = hash_mapping)]
pub struct HashMapping {
    pub md5: String,
    pub filename: String,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = hash_mapping)]
pub struct NewHashMapping {
    pub md5: Uuid,
    pub filename: String,
}
