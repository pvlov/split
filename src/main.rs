
use actix_web::{get, web, App, HttpServer, Responder};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};


const PGHOST: &str = "postgres";

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   
    let connection_opts = PgConnectOptions::new()
        .host(PGHOST);

    let _pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connection_opts)
        .await
        .expect("Failed to connect to Postgres DB");


    HttpServer::new(|| App::new().service(index).service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
