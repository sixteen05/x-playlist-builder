use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use rspotify::{model::TrackId, prelude::*};
use x_playlist_builder::{
    auth::SpotifyAuth,
    filter::{
        filter_by_condition, filter_condition_to_playlist_name,
        filter_removed_songs_with_no_avaliable_market, search_artists,
    },
    playlist::{create_or_get_playlist, get_all_playlist_created_by_user},
    util::fetch_all,
};

#[derive(Parser)]
#[command(name = "x-playlist-builder")]
#[command(about = "A tool to create playlists from your liked songs in Spotify", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all playlists created by the user
    ListPlaylists,
    /// Create or update a playlist from liked songs based on a condition
    CreatePlaylist {
        /// Condition name (e.g., "old-hindi", "artist")
        #[arg(short, long)]
        condition: String,
        /// Condition value (e.g., artist name for "artist" condition)
        #[arg(short, long)]
        value: String,
    },
    /// Remove unavailable tracks from liked songs
    RemoveDeletedTracks,
}

enum MenuAction {
    ListPlaylists,
    CreatePlaylist,
    RemoveDeletedTracks,
    Exit,
}

async fn list_playlists(spotify: &rspotify::AuthCodeSpotify) {
    let playlists_created_by_user = get_all_playlist_created_by_user(spotify).await;
    println!("Playlists for the user:");
    for playlist in playlists_created_by_user {
        println!("  - {} ({})", playlist.name, playlist.id);
    }
}

async fn create_playlist(
    spotify: &rspotify::AuthCodeSpotify,
    condition_name: String,
    condition_value: String,
    artist_name: String,
) {
    let playlist_name = filter_condition_to_playlist_name(&condition_name, &artist_name);

    println!("Creating/updating playlist: {}", playlist_name);
    let existing_playlist = create_or_get_playlist(spotify, playlist_name).await;

    let current_user_saved_tracks = fetch_all(spotify.current_user_saved_tracks(None)).await;
    let mut tracks: Vec<TrackId> = Vec::new();

    println!("Filtering liked songs...");
    for item in current_user_saved_tracks {
        let filter_res = filter_by_condition(&condition_name, &condition_value, item.track);
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
        println!("Adding {} new tracks to playlist...", tracks.len());
        let playable_ids: Vec<rspotify::model::PlayableId> = tracks
            .iter()
            .map(|track_id| rspotify::model::PlayableId::Track(track_id.clone()))
            .collect();
        spotify
            .playlist_add_items(existing_playlist.id, playable_ids, None)
            .await
            .unwrap();
        println!("Playlist updated successfully!");
    } else {
        println!("No new tracks to add.");
    }
}

async fn remove_deleted_tracks(spotify: &rspotify::AuthCodeSpotify) {
    let current_user_saved_tracks = fetch_all(spotify.current_user_saved_tracks(None)).await;
    let mut tracks: Vec<TrackId> = Vec::new();

    println!("Scanning for unavailable tracks...");
    for item in current_user_saved_tracks {
        let filter_res = filter_removed_songs_with_no_avaliable_market(&item.track);
        if filter_res {
            tracks.push(item.track.id.unwrap());
        }
    }

    if !tracks.is_empty() {
        println!("Removing {} unavailable tracks...", tracks.len());
        for chunk in tracks.chunks(50) {
            spotify
                .current_user_saved_tracks_delete(chunk.iter().cloned())
                .await
                .unwrap();
        }
        println!("Removed {} tracks from liked songs!", tracks.len());
    } else {
        println!("No unavailable tracks found.");
    }
}

fn show_main_menu() -> MenuAction {
    let options = vec![
        "List all playlists",
        "Create/update playlist from liked songs",
        "Remove unavailable tracks from liked songs",
        "Exit",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => MenuAction::ListPlaylists,
        1 => MenuAction::CreatePlaylist,
        2 => MenuAction::RemoveDeletedTracks,
        3 => MenuAction::Exit,
        _ => MenuAction::Exit,
    }
}

async fn get_create_playlist_inputs(
    spotify: &rspotify::AuthCodeSpotify,
) -> (String, String, String) {
    let conditions = vec!["artist", "old-hindi"];

    let condition_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a condition")
        .items(&conditions)
        .default(0)
        .interact()
        .unwrap();

    let condition = conditions[condition_selection].to_string();

    let (artist_id, artist_name) = if condition == "artist" {
        let search_query: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter artist name to search")
            .interact_text()
            .unwrap();

        println!("Searching for artists...");

        // Try to refresh token before searching
        if let Err(e) = spotify.refetch_token().await {
            println!("Note: Could not refresh token preemptively: {:?}", e);
        }

        let artists = match search_artists(spotify, &search_query, 10).await {
            Ok(artists) => artists,
            Err(e) => {
                println!("Search failed: {:?}", e);
                println!("Attempting to refresh token...");
                match spotify.refetch_token().await {
                    Ok(_) => {
                        println!("Token refreshed, retrying search...");
                        match search_artists(spotify, &search_query, 10).await {
                            Ok(artists) => artists,
                            Err(e) => {
                                println!("Search failed again: {:?}", e);
                                println!("Please restart the application to re-authenticate.");
                                return (String::new(), String::new(), String::new());
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to refresh token: {:?}", e);
                        println!("Please restart the application to re-authenticate.");
                        return (String::new(), String::new(), String::new());
                    }
                }
            }
        };

        if artists.is_empty() {
            println!("No artists found. Please try again.");
            return (String::new(), String::new(), String::new());
        }

        let artist_options: Vec<String> = artists
            .iter()
            .map(|a| {
                let genres = if a.genres.is_empty() {
                    "No genres".to_string()
                } else {
                    a.genres.join(", ")
                };
                format!("{} | Followers: {} | Genres: {}", a.name, a.followers, genres)
            })
            .collect();

        let artist_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an artist")
            .items(&artist_options)
            .default(0)
            .interact()
            .unwrap();

        let selected_artist = &artists[artist_selection];
        (selected_artist.id.clone(), selected_artist.name.clone())
    } else {
        (String::new(), String::new())
    };

    (condition, artist_id, artist_name)
}

async fn run_interactive_mode() {
    // Authenticate once at the start
    println!("Authenticating with Spotify...\n");
    let resp = SpotifyAuth::new().await;
    let spotify = resp.client;

    loop {
        println!("\n========================================");
        println!("  Spotify Playlist Builder");
        println!("========================================\n");

        let action = show_main_menu();

        println!();

        match action {
            MenuAction::ListPlaylists => {
                list_playlists(&spotify).await;
            }
            MenuAction::CreatePlaylist => {
                let (condition, artist_id, artist_name) = get_create_playlist_inputs(&spotify).await;
                if !condition.is_empty() {
                    create_playlist(&spotify, condition, artist_id, artist_name).await;
                }
            }
            MenuAction::RemoveDeletedTracks => {
                remove_deleted_tracks(&spotify).await;
            }
            MenuAction::Exit => {
                println!("Goodbye!");
                break;
            }
        }

        println!("\nPress Enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::ListPlaylists) => {
            let resp = SpotifyAuth::new().await;
            list_playlists(&resp.client).await;
        }
        Some(Commands::CreatePlaylist { condition, value }) => {
            let resp = SpotifyAuth::new().await;

            if condition == "artist" {
                println!("Searching for artists matching '{}'...", value);

                let artists = match search_artists(&resp.client, &value, 10).await {
                    Ok(artists) => artists,
                    Err(e) => {
                        println!("Search failed: {:?}", e);
                        println!("Attempting to refresh token...");
                        match resp.client.refetch_token().await {
                            Ok(_) => {
                                println!("Token refreshed, retrying search...");
                                match search_artists(&resp.client, &value, 10).await {
                                    Ok(artists) => artists,
                                    Err(e) => {
                                        println!("Search failed again: {:?}", e);
                                        return;
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Failed to refresh token: {:?}", e);
                                return;
                            }
                        }
                    }
                };

                if artists.is_empty() {
                    println!("No artists found for query: {}", value);
                    return;
                }

                println!("Top artist results:");
                for (idx, artist) in artists.iter().enumerate() {
                    let genres = if artist.genres.is_empty() {
                        "No genres".to_string()
                    } else {
                        artist.genres.join(", ")
                    };
                    println!("{}. {} | Followers: {} | Genres: {}", idx + 1, artist.name, artist.followers, genres);
                }

                println!("\nUsing top result: {}", artists[0].name);
                create_playlist(&resp.client, condition, artists[0].id.clone(), artists[0].name.clone()).await;
            } else {
                create_playlist(&resp.client, condition, value.clone(), String::new()).await;
            }
        }
        Some(Commands::RemoveDeletedTracks) => {
            let resp = SpotifyAuth::new().await;
            remove_deleted_tracks(&resp.client).await;
        }
        None => run_interactive_mode().await,
    }
}
