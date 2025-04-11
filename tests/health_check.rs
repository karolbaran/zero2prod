use std::{collections::HashMap, net::TcpListener};

use actix_web::web::Form;
use reqwest::StatusCode;
use zero2prod::FormData;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Filed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("asdf");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

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

#[tokio::test]
async fn subscribe_return_200_for_a_valid_form_data() {
    let app = spawn_app();
    let client = reqwest::Client::new();

    let body = FormData {
        name: "karol".to_string(),
        email: "karol%40gmail.com".to_string(),
    };

    let response = client
        .post(&format!("{}/subscriptions", &app))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&body)
        .send()
        .await
        .expect("Failed to execute request");

    dbg!(&response);
    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn subscribe_return_400_when_data_is_missing() {
    let app = spawn_app();
    let client = reqwest::Client::new();

    let test_case = vec![
        ("name=karol", "missing the email"),
        ("email=karolek", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_case {
        let response = client
            .post(&format!("{}/subscriptions", &app))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not faile with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
