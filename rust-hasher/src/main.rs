use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    hash
}
fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    let argon2 = Argon2::default();
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

fn main() {
    let pass: &str = "hello123";

    let hash = hash_password(pass);

    println!("Password: {}", pass);
    println!("Hash: {}", hash);
    let valid = verify_password(pass, &hash);
    println!("Correct password valid: {}", valid);
    let wrong = verify_password("wrong_password", &hash);
    println!("Wrong password valid: {}", wrong);
}
