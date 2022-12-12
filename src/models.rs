use diesel::prelude::*;

#[derive(Queryable)]
pub struct HashMapping {
    pub md5: Vec<u8>,
    pub filename: String,
}
