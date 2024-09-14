use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use tracing::debug;

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let hashed_password: String = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    debug!("Hashed Password: {:?}", &hashed_password);

    Ok(hashed_password)
}
