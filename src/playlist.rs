use rspotify::{clients::pagination::Paginator,model::{SimplifiedPlaylist, PlaylistId, FullPlaylist, PlaylistItem}, prelude::*, AuthCodeSpotify, ClientResult};
use futures::stream::TryStreamExt;

pub async fn create_or_get_playlist(spotify: AuthCodeSpotify) -> FullPlaylist {
    let user = spotify.me().await.unwrap();
    let playlists_created_by_user = get_all_playlist_created_by_user(&spotify).await;
    let mut playlist_id: Option<PlaylistId> = None;
    for playlist in playlists_created_by_user.iter() {
        if playlist.name == "Old hindi" {
            playlist_id = Some(playlist.id.clone());
            break;
        }
    };

    let created_updated_playlist_id = match playlist_id {
        Some(playlist_id) => playlist_id,
        None => {let playlist = spotify
        .user_playlist_create(
            &user.id,
            "Old hindi",
            Some(true), // Seems reverse
            None,
            Some("Playlist created by x-playlist-builder"),
        )
        .await
        .unwrap();
        playlist.id
    } 
    };

    return get_playlist_by_playlist_id(&spotify, &created_updated_playlist_id).await

}

pub async fn get_all_playlist_created_by_user(spotify: &AuthCodeSpotify) -> Vec<SimplifiedPlaylist>{
    let user = spotify.me().await.unwrap();
    let current_user_playlists = fetch_all(spotify.current_user_playlists()).await;
    return current_user_playlists.into_iter().filter(|p| p.owner.id == user.id).collect::<Vec<_>>()
}

pub async fn get_playlist_by_playlist_id(spotify: &AuthCodeSpotify, playlist_id: &PlaylistId) -> FullPlaylist{
    let user = spotify.me().await.unwrap();
    return spotify
    .user_playlist(&user.id, Some(&playlist_id), None)
    .await
    .unwrap(); 
}

async fn fetch_all<'a, T>(paginator: Paginator<'a, ClientResult<T>>) -> Vec<T> {
        paginator.try_collect::<Vec<_>>().await.unwrap()
}