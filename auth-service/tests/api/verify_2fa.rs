use auth_service::{
    domain::{LoginAttemptId, TwoFACode},
    ErrorResponse,
};
use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "email": random_email.clone(),
            "loginAttemptId": "1",
        }),
        serde_json::json!({
            "email": random_email.clone(),
            "2FACode": "4"
        }),
        serde_json::json!({
            "loginAttemptId": "1",
            "2FACode": "4"
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "email": random_email.clone(),
            "loginAttemptId": "1",
            "2FACode": "4"
        }),
        serde_json::json!({
            "email": random_email.clone(),
            "loginAttemptId": "1",
            "2FACode": "4"
        }),
    ];
    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
    }
}
