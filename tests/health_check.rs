use std::net::TcpListener;

use reqwest::StatusCode;
use sqlx::{Executor, PgPool, query};
use uuid::Uuid;
use zero2prod::{
    configuration::{DatabaseSettings, get_configuration},
    routes::FormData,
};

struct TestApp {
    address: String,
    db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let mut configuration = get_configuration().expect("Failed to read configuration file");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;
    let listener = TcpListener::bind("127.0.0.1:0").expect("Filed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener, connection_pool.clone()).expect("asdf");
    tokio::spawn(server);
    let address = format!("http://127.0.0.1:{}", port);
    TestApp {
        address,
        db_pool: connection_pool.clone(),
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let connection = PgPool::connect(&config.connection_settings_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    drop(connection);
    //Migrate database
    let connection_pool = PgPool::connect(&config.connection_settings())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

#[tokio::test]
async fn helth_check_works() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health", app.address))
        .send()
        .await
        .expect("asfasdfasfds");

    assert!(response.status().is_success());
    assert_eq!(StatusCode::OK, response.status())
}

#[tokio::test]
async fn subscribe_return_200_for_a_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = FormData {
        name: "karol3".to_string(),
        email: "karol3@gmail.com".to_string(),
    };

    let response = client
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&body)
        .send()
        .await
        .expect("Failed to execute request");

    let saved = query!("SELECT id, email, name, subscribed_at FROM subscriptions",)
        .fetch_all(&app.db_pool)
        .await;

    match saved {
        Ok(res) => {
            for row in res {
                println!(
                    "ID: {:?}, Email: {:?}, Name: {:?}, INSER_DATE: {:?}",
                    row.id, row.email, row.name, row.subscribed_at
                );
            }
        }
        Err(err) => {
            dbg!(err);
        }
    }
    assert_eq!(201, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_return_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_case = vec![
        ("name=karol", "missing the email"),
        ("email=karolek", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_case {
        let response = client
            .post(format!("{}/subscriptions", app.address))
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
