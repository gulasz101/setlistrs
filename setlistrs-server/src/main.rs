use actix_cors::Cors;
use actix_web::web::Json;
use actix_web::{get, App, HttpServer};
use setlistrs_types::{Cover, Setlist, Song};

#[get("/songs")]
async fn setlist() -> Json<Setlist> {
    let mut songs = Vec::new();

    songs.push(Song {
        name: "despacito / lambada / shivers / balaba boa".into(),
        cover: Some(Cover {
            url: "https://youtu.be/lvAvaUhDBNA".into(),
            description: "metro".into(),
        }),
        chords: "b G D A".into(),
    });

    songs.push(Song {
        name: "gasolina".into(),
        cover: Some(Cover {
            url: "https://youtu.be/jSTk8-ZJhd4".into(),
            description: "metro".into(),
        }),
        chords: "F F#".into(),
    });

    Json(Setlist { data: songs })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive(); // TODO -> setup cors proper way
        App::new().wrap(cors).service(setlist)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
