use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "password": "password123"
        }),
        serde_json::json!({
            "email": get_random_email()
        }),
        serde_json::json!({
            "email": "not-an-email",
            "password": "password123"
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}