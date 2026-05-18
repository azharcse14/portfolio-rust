use std::sync::Arc;

use domain::{
    ports::{PasswordHasher, TokenIssuer, UserRepository},
    value_objects::Email,
    DomainError,
};

use crate::{ApplicationError, ApplicationResult};

pub struct Login {
    users: Arc<dyn UserRepository>,
    hasher: Arc<dyn PasswordHasher>,
    tokens: Arc<dyn TokenIssuer>,
}

impl Login {
    pub fn new(
        users: Arc<dyn UserRepository>,
        hasher: Arc<dyn PasswordHasher>,
        tokens: Arc<dyn TokenIssuer>,
    ) -> Self {
        Self {
            users,
            hasher,
            tokens,
        }
    }

    pub async fn execute(&self, email: &str, password: &str) -> ApplicationResult<String> {
        let email = Email::parse(email)?;
        let user = self
            .users
            .find_by_email(&email)
            .await
            .map_err(|e| match e {
                DomainError::NotFound => ApplicationError::Domain(DomainError::Unauthorized),
                other => other.into(),
            })?;

        if !self.hasher.verify(&user.password_hash, password)? {
            return Err(ApplicationError::Domain(DomainError::Unauthorized));
        }

        let token = self.tokens.issue(user.id, 24 * 7)?;
        Ok(token)
    }
}
