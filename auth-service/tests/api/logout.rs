use crate::helpers::TestApp;
#[tokio::test]
pub async fn logout_returns_200() {
    let app = TestApp::new().await;
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 200);
}