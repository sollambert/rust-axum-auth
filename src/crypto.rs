use bcrypt::{DEFAULT_COST, hash, verify, BcryptError};

pub fn print_hash(text: &str) -> String {
    let hashed = hash(text, DEFAULT_COST).unwrap();
    println!("{}", hashed);
    hashed
}

pub fn check_hash(text: &str, hash: &str) -> bool {
    verify(text, hash).unwrap()
}