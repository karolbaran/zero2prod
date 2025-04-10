use std::net::TcpListener;

use reqwest::StatusCode;

#[tokio::test]
async fn helth_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health", address))
        .send()
        .await
        .expect("asfasdfasfds");

    assert!(response.status().is_success());
    assert_eq!(StatusCode::OK, response.status())
}
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Filed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("asdf");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
