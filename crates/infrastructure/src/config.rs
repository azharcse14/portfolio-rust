//! Centralized configuration loaded from environment variables.

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub resend_api_key: Option<String>,
    pub cloudinary_url: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://portfolio.db?mode=rwc".into()),
            jwt_secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret".into()),
            resend_api_key: std::env::var("RESEND_API_KEY").ok(),
            cloudinary_url: std::env::var("CLOUDINARY_URL").ok(),
        }
    }
}
