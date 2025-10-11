# x-playlist-builder

A Rust-based CLI tool for Spotify playlist management that creates smart playlists from your liked songs and removes unavailable tracks.

## Features

- **Smart Playlist Creation**: Automatically create playlists from your liked songs based on conditions:
  - **Artist**: Search for artists using Spotify's API and filter songs by exact artist match
  - **Old Hindi**: Hindi songs released before 1990, available only in India market
- **Cleanup Tool**: Remove unavailable tracks from your liked songs
- **Interactive Mode**: User-friendly menu-based interface for all operations
- **Token Caching**: Seamless authentication with automatic token validation and refresh

## Prerequisites

- Rust (install from [rustup.rs](https://rustup.rs))
- Spotify account
- Spotify Developer credentials

## Setup

### 1. Get Spotify API Credentials

Create a Spotify Developer app to obtain your `client_id` and `client_secret`:

1. Go to the [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)
2. Create a new app
3. Add `http://localhost:8080/callback` as a Redirect URI in your app settings

### 2. Configure Environment Variables

Create a `.env` file in the project root with your credentials:

```env
RSPOTIFY_CLIENT_ID=your_client_id_value
RSPOTIFY_CLIENT_SECRET=your_client_secret_value
```

### 3. First Run

On first run, the app will open your browser for Spotify authorization. After granting permission, tokens are automatically cached to `.spotify_token_cache.json` and will be reused on subsequent runs. The app automatically validates and refreshes tokens as needed.

## Usage

### Interactive Mode (Recommended)

Launch the interactive menu:

```bash
cargo run
```

The menu provides options to:
- List all your playlists
- Create/update playlists from liked songs with:
  - **Artist search**: Search and select from top matching artists with follower counts and genres
  - **Old Hindi**: Automatic filtering for classic Hindi songs
- Remove unavailable tracks from liked songs
- Return to main menu after each operation

### Direct CLI Commands

For automation or scripting, use direct commands:

```bash
# List all playlists
cargo run -- list-playlists

# Create/update playlist for a specific artist (searches and uses top result)
cargo run -- create-playlist --condition artist --value "arijit singh"

# Create/update playlist for old Hindi songs
cargo run -- create-playlist --condition old-hindi --value ""

# Remove unavailable tracks from liked songs
cargo run -- remove-deleted-tracks

# Show help
cargo run -- --help
```

## How It Works

- **Authentication**: Uses OAuth 2.0 with automatic token caching, validation, and refresh. Invalid tokens are automatically detected and re-authenticated.
- **Artist Filtering**: Uses Spotify's Search API to find artists, displays results with follower counts and genres, and filters songs by exact artist ID match for accuracy
- **Playlist Creation**: Fetches all liked songs, filters by condition, checks for duplicates, and adds new tracks in batches
- **Deduplication**: Automatically prevents duplicate tracks when updating existing playlists
- **Token Resilience**: Automatically retries failed API calls after refreshing tokens

## Development

```bash
# Build the project
cargo build

# Check code without building
cargo check
```

## Known Issues

- Playlists are created as public even when private flag is set (Spotify API limitation - [discussion](https://community.spotify.com/t5/Spotify-for-Developers/Api-to-create-a-private-playlist-doesn-t-work/td-p/5407807))
