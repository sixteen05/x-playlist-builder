# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust-based CLI tool for Spotify playlist management that creates playlists from liked songs based on conditions, and removes unavailable tracks from liked songs. Uses clap for CLI parsing, dialoguer for interactive menus, and rspotify for Spotify API integration.

## Build & Run Commands

- **Build**: `cargo build`
- **Run interactive mode**: `cargo run` (launches menu-based interface)
- **Run with help**: `cargo run -- --help`
- **Test**: `cargo test`
- **Check**: `cargo check`

## CLI Modes

### Interactive Mode (Default)
When run without arguments, launches an interactive menu with:
- List all playlists
- Create/update playlist (with guided prompts for condition selection)
- Remove unavailable tracks
- Exit option with return to main menu after each action

### Direct CLI Commands
- **List playlists**: `cargo run -- list-playlists`
- **Create/update playlist**: `cargo run -- create-playlist --condition <CONDITION> --value <VALUE>`
  - Example: `cargo run -- create-playlist --condition artist --value arijit`
  - Example: `cargo run -- create-playlist --condition old-hindi --value ""`
- **Remove deleted tracks**: `cargo run -- remove-deleted-tracks`

## Spotify Authentication Setup

Requires `.env` file with Spotify developer credentials:
```
RSPOTIFY_CLIENT_ID=your_client_id_value
RSPOTIFY_CLIENT_SECRET=your_client_secret_value
```

Redirect URI is hardcoded to `http://localhost:8080/callback` and must be configured in Spotify Developer Dashboard. On first run, the app will prompt for OAuth authorization in the terminal.

## Architecture

**Authentication Flow (auth.rs)**
- `SpotifyAuth::new()` handles OAuth flow, prompts user in terminal for authorization
- Required scopes: user-read-private, user-read-email, user-library-read, user-library-modify, playlist-modify-private, playlist-read-private

**Filter System (filter.rs)**
- Supported conditions:
  - `old-hindi`: Hindi songs released before 1990, only available in India market
  - `artist`: Songs by artist name (case-insensitive partial match)
- `filter_removed_songs_with_no_avaliable_market()`: Identifies unplayable tracks based on `is_playable` flag

**Playlist Management (playlist.rs)**
- `create_or_get_playlist()`: Checks if playlist exists before creating
- `get_all_playlist_created_by_user()`: Filters playlists owned by authenticated user
- All playlist operations return `FullPlaylist` which includes track items for deduplication

**Pagination Utility (util.rs)**
- `fetch_all()`: Wrapper around rspotify Paginator that collects all pages into Vec

**Main Flow (main.rs)**
- CLI uses clap for argument parsing with optional subcommands
- Interactive mode (`run_interactive_mode`): Menu-driven loop using dialoguer for user interaction
  - `show_main_menu()`: Displays menu options and returns selected action
  - `get_create_playlist_inputs()`: Prompts for condition and value selection
  - Returns to menu after each operation with "Press Enter to continue" prompt
- Direct command mode: Three commands (ListPlaylists, CreatePlaylist, RemoveDeletedTracks)
- CreatePlaylist flow:
  1. Parse CLI arguments or interactive inputs for condition and value
  2. Auth creates/refreshes Spotify client
  3. Fetches all liked songs using pagination
  4. Filters tracks by condition
  5. Checks for duplicates against existing playlist tracks
  6. Adds new tracks in batches

## Known Issues

- Playlists are created as public even when private flag is set (Spotify API bug)
- No error recovery if authentication fails mid-request
- Batch size for track removal is hardcoded to 50 in `remove_deleted_tracks_from_liked_playlist`
