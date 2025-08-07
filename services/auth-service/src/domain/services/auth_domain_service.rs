use crate::domain::entities::user::User;
use shared::features::errors::{SystemError, SystemResult};
use shared::features::helper::password_helper::PasswordHelper;

pub struct AuthDomainService;

impl AuthDomainService {
    pub fn validate_login_credentials(user: &User, password: &str) -> SystemResult<()> {
        // Check if user can log in (account status)
        user.can_login()?;

        // Verify password
        let is_valid = PasswordHelper::verify_hashed_string(password, user.password_hash.as_ref())
            .map_err(|e| SystemError::InternalError(e.to_string()))?;

        if !is_valid {
            return Err(SystemError::InvalidCredentials);
        }

        Ok(())
    }

    pub fn validate_new_password(password: &str) -> SystemResult<()> {
        PasswordHelper::validate(password).map_err(|errors| {
            let error_messages = errors.join(", ");
            SystemError::WeakPassword(error_messages)
        })?;
        Ok(())
    }

    pub fn hash_password(password: &str) -> SystemResult<String> {
        PasswordHelper::hash_string(password).map_err(|e| SystemError::InternalError(e.to_string()))
    }

    pub fn should_lock_account(failed_attempts: i32, max_attempts: i32) -> bool {
        failed_attempts >= max_attempts
    }

    pub fn calculate_lockout_duration(failed_attempts: i32) -> i64 {
        // Exponential backoff: 2^(attempts-3) minutes, max 60 minutes
        let base_minutes = 2_i64.pow((failed_attempts - 3).max(0) as u32);
        base_minutes.min(60)
    }
}
