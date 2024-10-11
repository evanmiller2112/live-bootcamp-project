use crate::helpers::TestApp;
#[tokio::test]
pub async fn verify_2fa_returns_200() {
    let app = TestApp::new().await;
    let response = app.verify_2fa().await;
    assert_eq!(response.status().as_u16(), 200);
}