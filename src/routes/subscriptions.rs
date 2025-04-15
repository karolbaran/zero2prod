use actix_web::{
    HttpResponse,
    body::BoxBody,
    post,
    web::{self, Data},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<FormData>, pool: Data<PgPool>) -> HttpResponse {
    let result = sqlx::query!(
        r#"
    INSERT INTO subscriptions (id,email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(res) => {
            dbg!(&res);
            HttpResponse::Created().body(res.rows_affected().to_string())
        }
        Err(err) => {
            dbg!(&err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
