use argon2::{
    Argon2,
    password_hash::{
        PasswordHash,
        PasswordHasher, PasswordVerifier, rand_core::OsRng, SaltString,
    },
};

pub fn generate_hash(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_ref(), &salt).unwrap().to_string()
}

pub fn verify_hash(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(&hash).unwrap();
    Argon2::default().verify_password(password.as_ref(), &parsed_hash).is_ok()
}