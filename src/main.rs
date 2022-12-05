use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use rspotify::{
    model::{PlayableId, TrackId},
    prelude::*,
};
use serde::Deserialize;
use x_playlist_builder::{
    auth::SpotifyAuth,
    filter::{filter_by_condition, filter_condition_to_playlist_name},
    playlist::{create_or_get_playlist, get_all_playlist_created_by_user},
    util::fetch_all,
};

#[get("/me/playlists")]
async fn get_all_playlists() -> impl Responder {
    let resp = SpotifyAuth::new().await;
    let spotify = resp.client;
    let playlists_created_by_user = get_all_playlist_created_by_user(&spotify).await;
    println!("Playlists for the user");
    println!("{:#?}", playlists_created_by_user);
    HttpResponse::Ok().body("Got all user playlists!")
}

#[derive(Deserialize)]
struct Condition {
    name: String,
    value: String,
}

#[get("/liked/create-update-playlist/condition/{name}/{value}")]
async fn liked_songs(info: web::Path<Condition>) -> impl Responder {
    let condition_name = &info.name;
    let condition_value = &info.value;
    let resp = SpotifyAuth::new().await;
    let spotify = resp.client;
    let playlist_name = filter_condition_to_playlist_name(condition_name, condition_value);
    let existing_playlist = create_or_get_playlist(&spotify, playlist_name).await;

    let current_user_saved_tracks = fetch_all(spotify.current_user_saved_tracks(None)).await;
    let mut tracks: Vec<TrackId> = Vec::new();
    for item in current_user_saved_tracks {
        let filter_res = filter_by_condition(condition_name, condition_value, item.track);
        if filter_res.state {
            let trackid = filter_res.track_id;
            let mut song_already_exists = false;
            for playlist_item in &existing_playlist.tracks.items {
                if playlist_item.track.as_ref().unwrap().id().unwrap().id() == trackid.id() {
                    song_already_exists = true;
                    break;
                }
            }
            if !song_already_exists {
                tracks.push(trackid);
            }
        }
    }

    if !tracks.is_empty() {
        spotify
            .playlist_add_items(
                &existing_playlist.id,
                tracks.iter().map(|track| track as &dyn PlayableId),
                None,
            )
            .await
            .unwrap();
    }

    HttpResponse::Ok().body("Got liked songs & created/updated playlist with songs!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_all_playlists).service(liked_songs))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
