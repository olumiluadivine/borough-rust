use shared::features::errors::{SystemError, SystemResult};
use uuid::Uuid;
use shared::utils::caching::CacheService;

#[derive(Clone)]
pub struct AuthCacheService {
    cache_service: CacheService,
}

impl AuthCacheService {
    pub fn new(cache_service: CacheService) -> Self {
        Self { cache_service }
    }

    pub async fn cache_user_session(&self, user_id: Uuid, access_token: &str) -> SystemResult<()> {
        let session_key = format!("session:{}", user_id);
        let token_key = format!("security:{}", access_token);

        // Store session mapping (user_id -> access_token)
        self.cache_service
            .set(&session_key, access_token, Some(3600))
            .await?;

        // Store token mapping (access_token -> user_id)
        self.cache_service
            .set(&token_key, user_id.to_string(), Some(3600))
            .await?;

        Ok(())
    }

    pub async fn get_user_from_token(&self, access_token: &str) -> SystemResult<Option<Uuid>> {
        let token_key = format!("security:{}", access_token);

        let user_id_str = self
            .cache_service
            .get::<String>(&token_key)
            .await?;

        match user_id_str {
            Some(val) => {
                let user_id = val
                    .parse::<Uuid>()
                    .map_err(|e| SystemError::InternalError(e.to_string()))?;
                Ok(Some(user_id))
            }
            None => Ok(None),
        }
    }

    pub async fn invalidate_user_session(&self, user_id: Uuid) -> SystemResult<()> {
        let session_key = format!("session:{}", user_id);

        // Get the access token from the session
        let access_token = self
            .cache_service
            .get::<String>(&session_key)
            .await?;

        // Delete the session key
        self.cache_service.delete(&session_key).await?;

        // Delete the token mapping if it exists
        if let Some(token) = access_token {
            let token_key = format!("security:{}", token);
            self.cache_service.delete(&token_key).await?;
        }

        Ok(())
    }

    pub async fn blacklist_token(&self, token: &str, expiry_seconds: i64) -> SystemResult<()> {
        let blacklist_key = format!("blacklist:{}", token);

        self.cache_service
            .set(&blacklist_key, "1", Some(expiry_seconds as u64))
            .await?;

        Ok(())
    }

    pub async fn is_token_blacklisted(&self, token: &str) -> SystemResult<bool> {
        let blacklist_key = format!("blacklist:{}", token);

        self.cache_service.exists(&blacklist_key).await
    }

    pub async fn refresh_session_ttl(&self, user_id: Uuid, ttl_seconds: i64) -> SystemResult<()> {
        let session_key = format!("session:{}", user_id);

        // Check if session exists and reset expiry time
        if self.cache_service.exists(&session_key).await? {
            self.cache_service.expire(&session_key, ttl_seconds).await?;
        }

        Ok(())
    }
}
