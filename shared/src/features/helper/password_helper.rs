use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use crate::features::errors::SystemError;

pub struct PasswordHelper;

impl PasswordHelper {
    pub fn validate(password: &str) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if password.len() < 8 {
            errors.push("Password must be at least 8 characters long".to_string());
        }

        if password.len() > 128 {
            errors.push("Password must not exceed 128 characters".to_string());
        }

        if !password.chars().any(|c| c.is_ascii_lowercase()) {
            errors.push("Password must contain at least one lowercase letter".to_string());
        }

        if !password.chars().any(|c| c.is_ascii_uppercase()) {
            errors.push("Password must contain at least one uppercase letter".to_string());
        }

        if !password.chars().any(|c| c.is_ascii_digit()) {
            errors.push("Password must contain at least one number".to_string());
        }

        if !password
            .chars()
            .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c))
        {
            errors.push("Password must contain at least one special character".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn hash_string(password: &str) -> Result<String, SystemError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| SystemError::HashingError(e.to_string()))?;
        Ok(password_hash.to_string())
    }

    pub fn verify_hashed_string(password: &str, hash: &str) -> Result<bool, SystemError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| SystemError::HashingError(e.to_string()))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}