
use crate::helpers::TestApp;

#[tokio::test]
async fn login_works()
{
    let app      = TestApp::new().await;
    let response = app.test_login().await;

    assert_eq!(response.status().as_u16(), 200);
//    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}
