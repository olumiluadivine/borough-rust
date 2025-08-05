use chrono::Utc;
use rand::{rng, RngCore};
use uuid::Uuid;
use crate::entities::enums::UserRole;
use crate::features::errors::SystemError;
use crate::features::security::jwt::JwtClaims;

pub struct JwtHelper;

impl JwtHelper {
    pub fn generate_access_token(
        user_id: Uuid,
        email: String,
        role: UserRole,
        permissions: Vec<String>,
        secret: &str,
        expiry_seconds: i64,
        issuer: &str,
        audience: &str,
        jti: Uuid
    ) -> Result<String, SystemError> {
        let now = Utc::now();
        let exp = (now + chrono::Duration::seconds(expiry_seconds)).timestamp() as usize;

        let claims = JwtClaims::new(
            user_id,
            email,
            role,
            permissions,
            issuer.to_string(),
            audience.to_string(),
            jti,
            exp
        );

        JwtHelper::generate_jwt(&claims, secret)
    }

    pub fn generate_secure_token() -> String {
        let mut rng = rng();
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        hex::encode(bytes)
    }

    pub fn generate_jti() -> Uuid {
        Uuid::new_v4()
    }

    fn generate_jwt(
        claims: &JwtClaims,
        secret: &str,
    ) -> Result<String, SystemError> {
        use jsonwebtoken::{encode, EncodingKey, Header};

        encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
            .map_err(|e| SystemError::TokenError(e.to_string()))
    }

    pub fn validate_jwt(
        token: &str,
        secret: &str,
    ) -> Result<JwtClaims, SystemError> {
        use jsonwebtoken::{decode, DecodingKey, Validation};

        let token_data = decode::<JwtClaims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default(),)
            .map_err(|e| SystemError::TokenError(e.to_string()))?;

        Ok(token_data.claims)
    }

    pub fn extract_user_id(token: &str, secret: &str) -> Result<Uuid, SystemError> {
        let claims = JwtHelper::validate_jwt(token, secret)?;
        Ok(claims.sub)
    }

    pub fn is_token_expired(claims: &JwtClaims) -> bool {
        let now = Utc::now().timestamp() as usize;
        claims.exp <= now
    }
}