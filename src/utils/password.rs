use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{Error, PasswordHash, PasswordHasher, SaltString, rand_core::OsRng},
};

pub fn hash_password(pass: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(pass.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}
pub fn verify_password(password: &str, hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
