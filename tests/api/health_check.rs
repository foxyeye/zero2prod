use crate::helpers::spawn_app;

#[actix_web::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    // let res = match response {
    //     Ok(r) => r,
    //     Err(e) => panic!("{}", e),
    // };

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
