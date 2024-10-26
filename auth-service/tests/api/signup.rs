use auth_service::routes::signup;
use crate::helpers;
use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
pub async fn should_return_422_if_malformed_input() {
    let app: TestApp = TestApp::new().await;
    let random_email: String = helpers::get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(&test_cases);
        assert_eq!(
            response.await.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}


#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = crate::helpers::TestApp::new().await;

    let valid_signup_data = serde_json::json!({
    "email": crate::helpers::get_random_email(),
    "password": "ValidPassword123!",
    "requires2FA": true
});

    let response = app.post_signup(&valid_signup_data).await;

    assert_eq!(response.status().as_u16(), 201);
}