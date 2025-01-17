use auth_service::services::HashmapUserStore;
use crate::helpers::{get_random_email, TestApp};


#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email(); // Call helper method to generate email

    let test_cases = [
        serde_json::json!({
            "email": random_email,
            "password": "password123",
            "requires_2fa": true
        }),
    ];

    // Test - new user - should respond 201: Created
    for test_case in test_cases.iter() {
        let response = (&app).post_signup(test_case).await;
        let response_code = response.status().as_u16();
        assert_eq!(
            response.status().as_u16(),
            201,
            "Failed for input: {:?}, response: {response_code}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email 

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires_2fa": true
        }),
        serde_json::json!({
            "email": "me@fakemail.biz",
            "requires_2fa": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": "Password123"
        })
    ];

    for test_case in test_cases.iter() {
        let response = (&app).post_signup(test_case).await;
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
    // The signup route should return a 400 HTTP status code if an invalid input is sent.
    // The input is considered invalid if:
    // - The email is empty or does not contain '@'
    // - The password is less than 8 characters

    // Create an array of invalid inputs. Then, iterate through the array and
    // make HTTP calls to the signup route. Assert a 400 HTTP status code is returned.
    let app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email

    let test_cases = [
        serde_json::json!({
            "email": "fake",
            "password": "password123",
            "requires_2fa": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": "short",
            "requires_2fa": true
        }),
    ];

    for test_case in test_cases.iter() {
        let response = (&app).post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;
    let random_email = get_random_email(); // Call helper method to generate email

    let test_cases = [
        serde_json::json!({
            "email": random_email,
            "password": "password123",
            "requires_2fa": true
        }),
    ];

    // Test - new user - should respond 201: Created
    for test_case in test_cases.iter() {
        let response = (&app).post_signup(test_case).await;
        let response_code = response.status().as_u16();
        assert_eq!(
            response.status().as_u16(),
            201,
            "Failed for input: {:?}, response: {response_code}",
            test_case
        );
    }
    
    // Test 2, same input, should fail with error 409 as the user was already created in Test 1.
    for test_case in test_cases.iter() {
        let response = (&app).post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            409,
            "Failed for input: {:?}",
            test_case
        );
    }
}