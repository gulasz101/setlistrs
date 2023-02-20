use std::env;

use actix_cors::Cors;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, HttpResponse};
use actix_web::{get, post, App, HttpServer};
use dotenvy::dotenv;
use setlistrs_types::{NewSetlist, Song, SongList, YTLink};
use sqlx::SqlitePool;

mod repository;

#[get("/songs")]
async fn setlist(pool: Data<SqlitePool>) -> HttpResponse {
    let mut songs = Vec::new();

    songs.push(Song {
        name: "despacito / lambada / shivers / balaba boa".into(),
        source: vec![],
        cover: Some(vec![YTLink {
            url: "https://youtu.be/lvAvaUhDBNA".into(),
            display_title: Some("metro".into()),
        }]),
        chords: "b G D A".into(),
    });

    songs.push(Song {
        name: "gasolina".into(),
        source: vec![],
        cover: Some(vec![YTLink {
            url: "https://youtu.be/jSTk8-ZJhd4".into(),
            display_title: None,
        }]),
        chords: "F F#".into(),
    });

    match crate::repository::list_all_songs(pool.get_ref()).await {
        Ok(songs) => HttpResponse::Ok().json(SongList { data: songs }),
        Err(_e) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/songs")]
async fn persist_song(song: Json<Song>, pool: Data<SqlitePool>) -> HttpResponse {
    match crate::repository::persist_song(pool.get_ref(), song.into_inner()).await {
        Ok(song) => HttpResponse::Created().json(song),
        Err(_e) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/setlists")]
async fn persist_setlist(pool: Data<SqlitePool>, new_setlist: Json<NewSetlist>) -> HttpResponse {
    match crate::repository::persist_setlist(pool.get_ref(), new_setlist.into_inner()).await {
        Ok(persisted_setlis) => HttpResponse::Created().json(persisted_setlis),
        Err(_e) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/setlists/{setlist_id}")]
async fn setlist_by_id(pool: Data<SqlitePool>, setlist_id: Path<i64>) -> HttpResponse {
    match crate::repository::obtain_setlist_by_id(pool.get_ref(), setlist_id.into_inner()).await {
        Ok(setlist_by_id) => HttpResponse::Ok().json(setlist_by_id),
        Err(_e) => HttpResponse::NotFound().into(),
    }
}
#[get("/setlists")]
async fn setlists_list(pool: Data<SqlitePool>) -> HttpResponse {
    match crate::repository::obtain_setlists_list(pool.get_ref()).await {
        Ok(setlist_list) => HttpResponse::Ok().json(setlist_list),
        Err(_e) => HttpResponse::InternalServerError().into(),
    }
}
#[delete("/setlists/{setlist_id}")]
async fn setlist_delete_by_id(pool: Data<SqlitePool>, setlist_id: Path<i64>) -> HttpResponse {
    match crate::repository::delete_setlist_by_id(pool.get_ref(), setlist_id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().into(),
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
            .service(persist_setlist)
            .service(setlist_by_id)
            .service(setlists_list)
            .service(setlist_delete_by_id)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await?;

    Ok(())
}
