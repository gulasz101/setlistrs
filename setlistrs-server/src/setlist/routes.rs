use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse, Responder,
};
use setlistrs_types::NewSetlist;
use sqlx::SqlitePool;

use crate::setlist::repository;

pub fn init(config: &mut ServiceConfig) {
    config
        .service(find_all)
        .service(find_by_id)
        .service(create)
        .service(delete);
}

#[get("/setlists")]
async fn find_all(pool: Data<SqlitePool>) -> impl Responder {
    match repository::find_all(pool.get_ref()).await {
        Ok(setlist_list) => HttpResponse::Ok().json(setlist_list),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}
#[get("/setlists/{setlist_id}")]
async fn find_by_id(pool: Data<SqlitePool>, setlist_id: Path<i64>) -> impl Responder {
    match repository::find_by_id(pool.get_ref(), setlist_id.into_inner()).await {
        Ok(setlist_by_id) => HttpResponse::Ok().json(setlist_by_id),
        Err(e) => HttpResponse::NotFound().body(format!("{}", e)),
    }
}
#[post("/setlists")]
async fn create(pool: Data<SqlitePool>, new_setlist: Json<NewSetlist>) -> impl Responder {
    match repository::create(pool.get_ref(), new_setlist.into_inner()).await {
        Ok(persisted_setlis) => HttpResponse::Created().json(persisted_setlis),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}
#[delete("/setlists/{setlist_id}")]
async fn delete(pool: Data<SqlitePool>, setlist_id: Path<i64>) -> impl Responder {
    match repository::delete(pool.get_ref(), setlist_id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}
