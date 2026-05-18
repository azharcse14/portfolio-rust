use std::sync::Arc;

use chrono::Utc;
use domain::{
    entities::User,
    ports::{PasswordHasher, UserRepository},
    value_objects::Email,
    DomainError,
};
use uuid::Uuid;

use crate::{ApplicationError, ApplicationResult};

/// Bootstrap-only: creates the first admin user. Refuses if any user already exists.
pub struct RegisterAdmin {
    users: Arc<dyn UserRepository>,
    hasher: Arc<dyn PasswordHasher>,
}

impl RegisterAdmin {
    pub fn new(users: Arc<dyn UserRepository>, hasher: Arc<dyn PasswordHasher>) -> Self {
        Self { users, hasher }
    }

    pub async fn execute(
        &self,
        email: &str,
        password: &str,
        name: Option<String>,
    ) -> ApplicationResult<Uuid> {
        if self.users.count().await? > 0 {
            return Err(ApplicationError::Domain(DomainError::Conflict(
                "admin already exists; setup is closed".into(),
            )));
        }
        if password.len() < 8 {
            return Err(ApplicationError::Invalid(
                "password must be at least 8 characters".into(),
            ));
        }

        let email = Email::parse(email)?;
        let user = User {
            id: Uuid::new_v4(),
            email,
            password_hash: self.hasher.hash(password)?,
            name,
            created_at: Utc::now(),
        };
        let id = user.id;
        self.users.create(&user).await?;
        Ok(id)
    }
}
