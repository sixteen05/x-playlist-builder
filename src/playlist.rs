use rspotify::{model::FullPlaylist, prelude::*, AuthCodeSpotify};

pub async fn create_playlist(spotify: AuthCodeSpotify) -> FullPlaylist {
    let user = spotify.me().await.unwrap();
    spotify
        .user_playlist_create(
            &user.id,
            "Old hindi",
            Some(true), // Seems reverse
            None,
            Some("Playlist created by x-playlist-builder"),
        )
        .await
        .unwrap()
}
