use crate::helpers::{get_random_email, TestApp};


#[tokio::test]
async fn should_return_201_if_valid_input() {
    //...
    todo!();

    // assert_eq!(response.status().as_u16(), 201);
    // 
    // let expected_response = SignupResponse {
    //     message: "User created successfully!".to_owned(),
    // };
    // 
    // // Assert that we are getting the correct response body!
    // assert_eq!(
    //     response
    //         .json::<SignupResponse>()
    //         .await
    //         .expect("Could not deserialize response body to UserBody"),
    //     expected_response
    // );
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email 

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "password": "g",
            "requires2FA": "true"
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