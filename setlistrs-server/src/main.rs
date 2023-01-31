use actix_cors::Cors;
use actix_web::web::Json;
use actix_web::HttpResponse;
use actix_web::{get, post, App, HttpServer};
use setlistrs_types::{Setlist, Song, YTLink};

#[get("/songs")]
async fn setlist() -> Json<Setlist> {
    let mut songs = Vec::new();

    songs.push(Song {
        name: "despacito / lambada / shivers / balaba boa".into(),
        cover: Some(vec![YTLink {
            url: "https://youtu.be/lvAvaUhDBNA".into(),
            display_title: Some("metro".into()),
        }]),
        chords: "b G D A".into(),
    });

    songs.push(Song {
        name: "gasolina".into(),
        cover: Some(vec![YTLink {
            url: "https://youtu.be/jSTk8-ZJhd4".into(),
            display_title: None,
        }]),
        chords: "F F#".into(),
    });

    Json(Setlist { data: songs })
}

#[post("/songs")]
async fn persist_song() -> HttpResponse {
    HttpResponse::Created().body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive(); // TODO -> setup cors proper way
        App::new().wrap(cors).service(setlist).service(persist_song)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
