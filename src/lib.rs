use std::{net::TcpListener};

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use sqlx::{PgPool};
use uuid::Uuid;
use chrono::Utc;

pub mod configuration;
pub mod routes;
pub mod startup;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

#[derive(serde::Deserialize)]
pub struct LoginData {
    name: String,
    password: String,
}

async fn subscribe(
    data: web::Json<LoginData>,
    connection: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    sqlx::query!(
        r#"
            INSERT INTO users (id, name, password,created_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        data.name,
        data.password,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    
    Ok(HttpResponse::Ok().finish())
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/users", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
