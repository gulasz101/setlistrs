use std::env;

use actix_cors::Cors;
use actix_web::web::Data;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;

mod setlist;
mod song;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let dsn = env::var("DATABASE_URL").expect("DATABASE_URL has to be set");
    let pool = sqlx::SqlitePool::connect(&dsn).await?;

    let host = match env::var("SETLISTRS_HOST") {
        Ok(host) => host,
        Err(_) => "0.0.0.0".into(),
    };

    let port: u16 = match env::var("SETLISTRS_PORT") {
        Ok(port) => port.parse().expect("port has to be u16"),
        Err(_) => 8081,
    };

    HttpServer::new(move || {
        let cors = Cors::permissive(); // TODO -> setup cors proper way
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(cors)
            .configure(song::init)
            .configure(setlist::init)
    })
    .bind((host, port))?
    .run()
    .await?;

    Ok(())
}
