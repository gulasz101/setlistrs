use std::env;

use actix_cors::Cors;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use actix_web::{get, post, App, HttpServer};
use dotenvy::dotenv;
use setlistrs_types::{Setlist, SongPersist, YTLinkPersist};
use sqlx::SqlitePool;

mod repository;

#[get("/songs")]
async fn setlist() -> Json<Setlist> {
    let mut songs = Vec::new();

    songs.push(SongPersist {
        name: "despacito / lambada / shivers / balaba boa".into(),
        source: vec![],
        cover: Some(vec![YTLinkPersist {
            url: "https://youtu.be/lvAvaUhDBNA".into(),
            display_title: Some("metro".into()),
        }]),
        chords: "b G D A".into(),
    });

    songs.push(SongPersist {
        name: "gasolina".into(),
        source: vec![],
        cover: Some(vec![YTLinkPersist {
            url: "https://youtu.be/jSTk8-ZJhd4".into(),
            display_title: None,
        }]),
        chords: "F F#".into(),
    });

    Json(Setlist { data: songs })
}

#[post("/songs")]
async fn persist_song(song: Json<SongPersist>, pool: Data<SqlitePool>) -> HttpResponse {
    match crate::repository::persist_song(pool.get_ref(), song.into_inner()).await {
        Ok(song) => HttpResponse::Created().json(song),
        Err(_e) => HttpResponse::InternalServerError().into(),
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let dsn = env::var("DATABASE_URL").expect("DATABASE_URL has to be set");
    let pool = sqlx::SqlitePool::connect(&dsn).await?;

    HttpServer::new(move || {
        let cors = Cors::permissive(); // TODO -> setup cors proper way
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(cors)
            .service(setlist)
            .service(persist_song)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await?;

    Ok(())
}
