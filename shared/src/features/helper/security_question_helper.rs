use crate::features::errors::SystemError;
use crate::features::helper::password_helper::PasswordHelper;

pub struct SecurityHelper;

impl SecurityHelper {
    pub fn hash_security_answer(answer: &str) -> Result<String, SystemError> {
        let normalized = answer.trim().to_lowercase();
        PasswordHelper::hash_string(&normalized)
    }

    pub fn verify_security_answer(answer: &str, hash: &str) -> Result<bool, SystemError> {
        let normalized = answer.trim().to_lowercase();
        PasswordHelper::verify_hashed_string(&normalized, hash)
    }
}