use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures::stream::TryStreamExt;
use futures_util::pin_mut;
use rspotify::{
    model::{track, PlayableId, PlaylistItem, TrackId},
    prelude::*,
};
use serde::Deserialize;
use x_playlist_builder::{
    auth::init_spotify,
    playlist::{create_or_get_playlist, get_all_playlist_created_by_user},
};

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

#[get("/me/playlists")]
async fn get_all_playlists() -> impl Responder {
    let mut spotify = init_spotify();
    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();
    let spotify_client = spotify.clone();
    let playlists_created_by_user = get_all_playlist_created_by_user(&spotify_client).await;
    println!("Playlists for the user");
    println!("{:#?}", playlists_created_by_user);
    return HttpResponse::Ok().body("Got all user playlists!");
}

#[get("/liked/old-songs/create-update-playlist")]
async fn liked_songs() -> impl Responder {
    let mut spotify = init_spotify();
    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();
    let spotify_client = spotify.clone();
    let fullplaylist = create_or_get_playlist(spotify_client).await;

    let stream = spotify.current_user_saved_tracks(None);
    let mut tracks: Vec<TrackId> = Vec::new();

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
            let mut song_already_exists = false;
            for item in &fullplaylist.tracks.items {
                if item.track.as_ref().unwrap().id().unwrap().id() == trackid.id() {
                    song_already_exists = true;
                    break;
                }
            }
            if !song_already_exists {
                tracks.push(trackid.clone());
            }
        }
    }

    if tracks.len() != 0 {
        spotify
            .playlist_add_items(
                &fullplaylist.id,
                tracks.iter().map(|track| track as &dyn PlayableId),
                None,
            )
            .await
            .unwrap();
    }

    return HttpResponse::Ok()
        .body("Got liked songs & created the first playlist by x-playlist-builder!");
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
            .service(get_all_playlists)
            .service(liked_songs)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
