use super::AuthCacheService;
use redis::cluster::ClusterClient;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_cache_and_get_user_session() {
    let redis_nodes = vec!["redis://127.0.0.1:7000"];
    let client = ClusterClient::new(redis_nodes).expect("Failed to create Redis ClusterClient");
    let service = AuthCacheService::new(Arc::new(client));

    let user_id = Uuid::new_v4();
    let access_token = "test_token_123";

    service.cache_user_session(user_id, access_token).await.expect("Failed to cache session");

    let fetched_user = service.get_user_from_token(access_token).await.expect("Failed to get user from security");

    assert_eq!(fetched_user, Some(user_id));
}

#[tokio::test]
async fn test_invalidate_user_session() {
    let redis_nodes = vec!["redis://127.0.0.1:7000"];
    let client = ClusterClient::new(redis_nodes).expect("Failed to create Redis ClusterClient");
    let service = AuthCacheService::new(Arc::new(client));

    let user_id = Uuid::new_v4();
    let access_token = "invalidate_test_token";

    service.cache_user_session(user_id, access_token).await.expect("Failed to cache session");

    // Invalidate session
    service.invalidate_user_session(user_id).await.expect("Failed to invalidate session");

    // Should return None now
    let fetched_user = service.get_user_from_token(access_token).await.expect("Failed to get user after invalidation");

    assert!(fetched_user.is_none(), "Session should be invalidated but token still resolves to user");
}

#[tokio::test]
async fn test_token_blacklist() {
    let redis_nodes = vec!["redis://127.0.0.1:7000"];
    let client = ClusterClient::new(redis_nodes).expect("Failed to create Redis ClusterClient");
    let service = AuthCacheService::new(Arc::new(client));

    let token = "blacklist_token_001";

    // Ensure security is not blocklisted initially
    let is_blacklisted = service.is_token_blacklisted(token).await.expect("Blacklist check failed");
    assert!(!is_blacklisted, "Token should not be blacklisted initially");

    // Blacklist security
    service.blacklist_token(token, 60).await.expect("Failed to blacklist security");

    // Should now be blocklisted
    let is_blacklisted = service.is_token_blacklisted(token).await.expect("Blacklist check after insertion failed");
    assert!(is_blacklisted, "Token should be blacklisted but is not");
}
