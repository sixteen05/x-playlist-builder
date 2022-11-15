use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures::stream::TryStreamExt;
use futures_util::pin_mut;
use rspotify::{
    model:: PlayableId,
    prelude::*,
};
use serde::Deserialize;
use x_playlist_builder::{auth::init_spotify, playlist::create_playlist};

#[derive(Debug, Deserialize, Clone)]
pub struct AuthCallbackRequest {
    code: String,
}

#[get("/")]
async fn _index() -> impl Responder {
    let spotify = init_spotify();
    let auth_url = spotify.get_authorize_url(true).unwrap();
    return auth_url;
}

#[get("/callback")]
async fn _callback(info: web::Query<AuthCallbackRequest>) -> impl Responder {
    let mut spotify = init_spotify();
    let code = &info.code;
    spotify.request_token(code).await.unwrap();
    return HttpResponse::Ok().body("Got the code & access token!");
}

#[get("/liked/old-songs/create-playlist")]
async fn liked_songs() -> impl Responder {
    let mut spotify = init_spotify();
    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();
    let spotify_client = spotify.clone();
    let playlist = create_playlist(spotify_client).await;

    let stream = spotify.current_user_saved_tracks(None);
    let mut tracks = Vec::new();

    pin_mut!(stream);
    while let Some(item) = stream.try_next().await.unwrap() {
        let r = item.track.album.release_date.as_ref();
        let release_year = r.unwrap().split("-").next().unwrap();
        let year_val = release_year.parse::<i32>().unwrap();
        if year_val < 1990
            && item.track.available_markets.len() == 1
            && item.track.available_markets[0] == "IN"
        {
            println!("* {}, Year - {}", item.track.name, release_year);
            let trackid = item.track.id.unwrap();
            tracks.push(trackid);
        }
    }

    spotify
        .playlist_add_items(
            &playlist.id,
            tracks.iter().map(|track| track as &dyn PlayableId),
            None,
        )
        .await
        .unwrap();

    return HttpResponse::Ok().body("Got liked songs & created the first playlist by x-playlist-builder!");
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .service(index)
            .service(echo)
            // .service(callback)
            .service(liked_songs)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
