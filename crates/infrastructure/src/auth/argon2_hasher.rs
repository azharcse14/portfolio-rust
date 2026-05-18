use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher as _, PasswordVerifier, SaltString},
    Argon2,
};
use domain::{ports::PasswordHasher, DomainError, DomainResult};

pub struct Argon2Hasher {
    argon: Argon2<'static>,
}

impl Argon2Hasher {
    pub fn new() -> Self {
        Self {
            argon: Argon2::default(),
        }
    }
}

impl Default for Argon2Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordHasher for Argon2Hasher {
    fn hash(&self, plain: &str) -> DomainResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        self.argon
            .hash_password(plain.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| DomainError::Internal(format!("hash: {e}")))
    }

    fn verify(&self, hash: &str, plain: &str) -> DomainResult<bool> {
        let parsed =
            PasswordHash::new(hash).map_err(|e| DomainError::Internal(format!("parse: {e}")))?;
        Ok(self.argon.verify_password(plain.as_bytes(), &parsed).is_ok())
    }
}
