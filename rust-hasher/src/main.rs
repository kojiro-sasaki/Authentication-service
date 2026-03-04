use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
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

fn main() {
    let pass: &str = "hello123";

    let hash = hash_password(pass);

    println!("Password: {}", pass);
    println!("Hash: {}", hash);
}
