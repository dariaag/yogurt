use bcrypt::{hash, DEFAULT_COST};

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()//handle error
}
