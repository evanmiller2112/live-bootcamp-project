use auth_service::{utils::constants::JWT_COOKIE_NAME,
                   ErrorResponse};
use reqwest::Url;
use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    todo!()
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    todo!()
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;
    
    let verify_result = app.verify_token().await;
    assert_eq!(
        verify_result.status().as_u16(),
        400,
    );
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let verify_result = app.verify_token().await;
    assert_eq!(
        verify_result.status().as_u16(),
        401,
    );


}