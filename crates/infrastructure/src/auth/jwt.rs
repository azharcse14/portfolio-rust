use chrono::{Duration, Utc};
use domain::{ports::TokenIssuer, DomainError, DomainResult};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Claims {
    /// Subject — the user uuid as string.
    sub: String,
    /// Expiry, unix timestamp seconds.
    exp: i64,
    /// Issued-at, unix timestamp seconds.
    iat: i64,
}

pub struct JwtIssuer {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtIssuer {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }
}

impl TokenIssuer for JwtIssuer {
    fn issue(&self, user_id: Uuid, ttl_hours: i64) -> DomainResult<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(ttl_hours);
        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| DomainError::Internal(format!("jwt encode: {e}")))
    }

    fn verify(&self, token: &str) -> DomainResult<Uuid> {
        let data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|_| DomainError::Unauthorized)?;
        Uuid::parse_str(&data.claims.sub)
            .map_err(|e| DomainError::Internal(format!("bad sub: {e}")))
    }
}
