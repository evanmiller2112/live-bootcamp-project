use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let password = "abcABC123".to_string();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": password,
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201, "Signup Failed: {}", response.text().await.unwrap());

    // Verify that the signup API has created a user and returned the expected status code.
    //println!("Signup response: {:?}", response);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": password,
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200, "Login After signup Failed - didn't return 200");
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");
    assert!(!auth_cookie.value().is_empty(), "Auth cookie is empty");

    let token = auth_cookie.value();
    let verify_token_body = serde_json::json!({
        "token": &token,
    });
    let response = app.post_verify_token(&verify_token_body).await;
    assert_eq!(response.status().as_u16(), 200, "Verify Token Failed: {}", response.text().await.unwrap());
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;
    let token: &str = "henlo";
    let verify_token_body = serde_json::json!({
        "token": &token,
    });
    let response = app.post_verify_token(&verify_token_body).await;
    assert_eq!(response.status().as_u16(), 401, "Verify Token succeeded or failed in an unexpected way.");

}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = vec![
        serde_json::json!({
            "token": true,
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases {
        let response = app.post_verify_token(&test_case).await;
        assert_eq!(response.status().as_u16(), 422);
    }
}

