use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use rspotify::{
    model::{PlayableId, TrackId},
    prelude::*,
};
use x_playlist_builder::{
    auth::SpotifyAuth,
    playlist::{create_or_get_playlist, get_all_playlist_created_by_user}, util::fetch_all,
};

#[get("/me/playlists")]
async fn get_all_playlists() -> impl Responder {
    let resp = SpotifyAuth::new().await;
    let spotify = resp.client;
    let playlists_created_by_user = get_all_playlist_created_by_user(&spotify).await;
    println!("Playlists for the user");
    println!("{:#?}", playlists_created_by_user);
    return HttpResponse::Ok().body("Got all user playlists!");
}

#[get("/liked/old-songs/create-update-playlist")]
async fn liked_songs() -> impl Responder {
    let resp = SpotifyAuth::new().await;
    let spotify = resp.client;
    let fullplaylist = create_or_get_playlist(&spotify).await;

    let current_user_saved_tracks = fetch_all(spotify.current_user_saved_tracks(None)).await;
    let mut tracks: Vec<TrackId> = Vec::new();

    for item in current_user_saved_tracks{
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
                tracks.push(trackid);
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
        .body("Got liked songs & created/updated old songs playlist by x-playlist-builder!");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_all_playlists).service(liked_songs))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
