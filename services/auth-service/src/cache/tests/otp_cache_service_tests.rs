use super::OtpCacheService;
use redis::cluster::ClusterClient;
use std::sync::Arc;

#[tokio::test]
async fn test_otp_store_and_get() {
    let redis_nodes = vec![
        "redis://127.0.0.1:7000",  // Adjust this to your Redis cluster nodes
    ];
    let client = ClusterClient::new(redis_nodes).expect("Failed to create Redis ClusterClient");
    let service = OtpCacheService::new(Arc::new(client), 5, 3);

    let identifier = "test_user";
    let otp_code = "123456";

    service.store_otp(identifier, otp_code, 1).await.expect("Failed to store OTP");

    let retrieved = service.get_otp(identifier).await.expect("Failed to get OTP");

    assert_eq!(retrieved, Some(otp_code.to_string()));
}

#[tokio::test]
async fn test_otp_invalidate() {
    let redis_nodes = vec![
        "redis://127.0.0.1:7000",
    ];
    let client = ClusterClient::new(redis_nodes).expect("Failed to create Redis ClusterClient");
    let service = OtpCacheService::new(Arc::new(client), 5, 3);

    let identifier = "test_user_invalidate";
    let otp_code = "654321";

    service.store_otp(identifier, otp_code, 1).await.expect("Failed to store OTP");
    service.invalidate_otp(identifier).await.expect("Failed to invalidate OTP");

    let retrieved = service.get_otp(identifier).await.expect("Failed to get OTP after invalidation");

    assert_eq!(retrieved, None);
}

#[tokio::test]
async fn test_otp_rate_limit() {
    let redis_nodes = vec![
        "redis://127.0.0.1:7000",
    ];
    let client = ClusterClient::new(redis_nodes).expect("Failed to create Redis ClusterClient");
    let service = OtpCacheService::new(Arc::new(client), 1, 2); // 1-minute window, max 2 requests

    let identifier = "test_user_rate_limit";

    // First, request should pass
    service.check_otp_rate_limit(identifier).await.expect("First rate-limit check failed");

    // Second, request should pass
    service.check_otp_rate_limit(identifier).await.expect("Second rate-limit check failed");

    // Third request should fail (rate limit exceeded)
    let result = service.check_otp_rate_limit(identifier).await;

    assert!(result.is_err(), "Expected rate-limit error on third request");
}
