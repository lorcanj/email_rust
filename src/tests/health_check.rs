//! tests/health_check.rs

#[actix_re::test]
async fn health_check_works() {
    spawn_app().await.expect("Failed to spawn our app");
    
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(O), response.content_length());
}

async fn spawn_app() -> std::io::Result<()> {
    todo!()
}