use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse, Responder,
};
use setlistrs_types::{Song, SongList};
use sqlx::SqlitePool;

use crate::song::repository;

pub fn init(config: &mut ServiceConfig) {
    config.service(find_all).service(create).service(delete);
}

#[get("/songs")]
async fn find_all(pool: Data<SqlitePool>) -> impl Responder {
    match repository::find_all(pool.get_ref()).await {
        Ok(songs) => HttpResponse::Ok().json(SongList { data: songs }),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}
#[post("/songs")]
async fn create(song: Json<Song>, pool: Data<SqlitePool>) -> impl Responder {
    match repository::create(pool.get_ref(), song.into_inner()).await {
        Ok(song) => HttpResponse::Created().json(song),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}
#[delete("/songs/{song_id}")]
async fn delete(pool: Data<SqlitePool>, song_id: Path<i64>) -> impl Responder {
    match repository::soft_delete(pool.get_ref(), song_id.into_inner()).await {
        Ok(rows_affected) => match rows_affected {
            1 => HttpResponse::NoContent(),
            _ => HttpResponse::BadRequest(),
        }
        .finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}
