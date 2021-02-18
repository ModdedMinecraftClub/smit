use crate::response_error::GenericResult;
use argon2::{hash_raw, verify_raw, Config};
use rand::rngs::OsRng;
use rand::RngCore;
use std::convert::TryInto;

//if you're updating this, modify schema.sql appropriately (probably via adding a migration)
pub const HASH_LENGTH_BYTES: usize = 32;
pub const SALT_LENGTH_BYTES: usize = 16;

pub struct HashAndSalt {
    pub hash: [u8; HASH_LENGTH_BYTES],
    pub salt: [u8; SALT_LENGTH_BYTES],
}

#[inline]
pub fn hash_password(password: &str) -> GenericResult<HashAndSalt> {
    let mut salt = [0; SALT_LENGTH_BYTES];
    OsRng.fill_bytes(&mut salt[0..]);
    let hash = hash_raw(password.as_bytes(), &salt, &config())?
        .try_into()
        .expect("Hash was not of hash length bytes length.");

    Ok(HashAndSalt { hash, salt })
}

pub fn verify_hash(hash: &[u8], salt: &[u8], password: &str) -> argon2::Result<bool> {
    Ok(verify_raw(password.as_bytes(), salt, hash, &config())?)
}

#[inline]
fn config<'a>() -> Config<'a> {
    Config {
        hash_length: HASH_LENGTH_BYTES as u32,
        ..Config::default()
    }
}

#[test]
fn test_password_hashing() {
    let password = "This string is a password.";
    let hash_and_salt = hash_password(password).expect("failed to hash");
    assert!(
        verify_hash(&hash_and_salt.hash, &hash_and_salt.salt, password).expect("failed to verify")
    );
    assert!(!verify_hash(
        &hash_and_salt.hash,
        &hash_and_salt.salt,
        "This string is the wrong password."
    )
    .expect("failed to verify"));
}
