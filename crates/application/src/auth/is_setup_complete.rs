use std::sync::Arc;

use domain::ports::UserRepository;

use crate::ApplicationResult;

pub struct IsSetupComplete {
    users: Arc<dyn UserRepository>,
}

impl IsSetupComplete {
    pub fn new(users: Arc<dyn UserRepository>) -> Self {
        Self { users }
    }

    pub async fn execute(&self) -> ApplicationResult<bool> {
        Ok(self.users.count().await? > 0)
    }
}
