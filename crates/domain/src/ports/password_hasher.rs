use crate::DomainResult;

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, plain: &str) -> DomainResult<String>;
    fn verify(&self, hash: &str, plain: &str) -> DomainResult<bool>;
}
