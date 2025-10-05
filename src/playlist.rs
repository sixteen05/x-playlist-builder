use rspotify::{
    model::{FullPlaylist, PlaylistId, SimplifiedPlaylist},
    prelude::*,
    AuthCodeSpotify,
};

use crate::util::fetch_all;

pub async fn create_or_get_playlist(
    spotify: &AuthCodeSpotify,
    playlist_name: String,
) -> FullPlaylist {
    let playlists_created_by_user = get_all_playlist_created_by_user(spotify).await;
    let mut playlist_id: Option<&PlaylistId> = None;
    for playlist in playlists_created_by_user.iter() {
        if playlist.name == playlist_name {
            playlist_id = Some(&playlist.id);
            break;
        }
    }

    let playlist_create;
    let created_updated_playlist_id = match playlist_id {
        Some(playlist_id) => playlist_id,
        None => {
            playlist_create = create_playlist(spotify, playlist_name).await;
            &playlist_create.id
        }
    };

    return get_playlist_by_playlist_id(spotify, created_updated_playlist_id).await;
}

pub async fn get_all_playlist_created_by_user(
    spotify: &AuthCodeSpotify,
) -> Vec<SimplifiedPlaylist> {
    let user = spotify.me().await.unwrap();
    let current_user_playlists = fetch_all(spotify.current_user_playlists()).await;
    current_user_playlists
        .into_iter()
        .filter(|p| p.owner.id == user.id)
        .collect::<Vec<_>>()
}

pub async fn get_playlist_by_playlist_id(
    spotify: &AuthCodeSpotify,
    playlist_id: &PlaylistId<'_>,
) -> FullPlaylist {
    let user = spotify.me().await.unwrap();
    return spotify
        .user_playlist(user.id, Some(playlist_id.clone()), None)
        .await
        .unwrap();
}

pub async fn create_playlist(spotify: &AuthCodeSpotify, playlist_name: String) -> FullPlaylist {
    let user = spotify.me().await.unwrap();
    return spotify
        .user_playlist_create(
            user.id,
            &playlist_name,
            Some(false), // Private doesn't work, creates a public one - https://community.spotify.com/t5/Spotify-for-Developers/Api-to-create-a-private-playlist-doesn-t-work/td-p/5407807
            None,
            Some("Playlist created by x-playlist-builder"),
        )
        .await
        .unwrap();
}
