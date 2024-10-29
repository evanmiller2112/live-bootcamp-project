use auth_service::{utils::constants::JWT_COOKIE_NAME,
                   ErrorResponse};
use reqwest::Url;
use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    // TODO this shouldn't work .. I think we need to make an actual cookie here?
    let app = TestApp::new().await;
    

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
        200,
    );
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;
    
    //generate random email
    let random_email = get_random_email();
    let password = "abcABC123".to_owned();

    //generate signup body using serde::json
    // TODO this confirmation shouldn't be necessary, should be validated in
    // client-side code since it will be parsed prior to use
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": password,
        "requires2FA": false,
    });
    
    // Post signup, expect 201 or throw error.
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 
               201, 
               "Signup failed - expected 201, with input: {}", signup_body.to_string());
    
    // Login
    let login_body = serde_json::json!({
        "email": random_email,
        "password": password,
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 
               200, 
               "Login failed - expected 200, got {}",
               response.status().as_u16());

    let auth_cookie = 
        response.cookies()
            .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
            .expect("No auth cookie found.");
    
    assert!(!auth_cookie.value().is_empty(), "Auth cookie value is empty but it shouldn't be.");

    // post logout, should work.
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(),
               200,
               "Login failed - expected 200, got {}",
               response.status().as_u16());

    // post logout a second time, shouldn't work this time
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(),
            400,
            "Logout failed - expected 400, got {}",
            response.status().as_u16());

}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;
    
    let response = app.post_logout().await;
    assert_eq!(
        response.status().as_u16(),
        400,
        "The API did not return a 400 BAD REQUEST",
    );

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME);

    assert!(auth_cookie.is_none());

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Missing auth token".to_owned()
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

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(),
               401,
               "Expectd 401 but got {}",
               response.status().as_u16());

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME);

    assert!(auth_cookie.is_none());

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Invalid auth token".to_owned()
    );


}