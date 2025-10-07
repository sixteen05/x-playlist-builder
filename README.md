# x-playlist-builder

## Current State
WIP :construction:

A tool to create playlist from your liked songs in Spotify. Currently couple condition supported -
- 'old-hindi' hindi songs released before 1990
- 'artist' songs by artist name.

- Remove songs from liked songs list that are no longer available in your region. The API takes your country from the 
access token which is bascially the country your account is registered in.
More things to come ...

## Setup

Please note, things might not work as expected at times currently ...

You would need to create dev app on spotify to get credentials `client_id` & `client_secret`.

App can be created trhough the spotify developer dashboard. You can follow this [guide](https://developer.spotify.com/documentation/general/guides/authorization/app-settings/).
Ensure that you add `Redirect URI` as `http://localhost:8080/callback` since this program has that value hardcoded.

After getting the credentials, create `.env` file where you have cloned this project. Replace the values below with your credentials -
```
RSPOTIFY_CLIENT_ID=your_client_id_value
RSPOTIFY_CLIENT_SECRET=your_client_secret_value
```

## Running

- Ensure that you have `Rust` installed.

### Interactive Mode (Recommended)

Simply run:
```bash
cargo run
```

This will launch an interactive menu where you can:
- List all your playlists
- Create/update playlists from liked songs (with guided prompts)
- Remove unavailable tracks from liked songs
- Navigate back to the main menu after each action

On first run, the app will prompt you to authorize it in Spotify. Follow the instructions in the terminal.

### Direct CLI Commands

You can also use direct commands without the interactive menu:

**List all your playlists:**
```bash
cargo run -- list-playlists
```

**Create/update a playlist from liked songs:**
```bash
# Create playlist for a specific artist
cargo run -- create-playlist --condition artist --value arijit

# Create playlist for old Hindi songs
cargo run -- create-playlist --condition old-hindi --value ""
```

**Remove unavailable tracks from liked songs:**
```bash
cargo run -- remove-deleted-tracks
```

**View help:**
```bash
cargo run -- --help
```

## Known Issues
- Currently, the playlist created is public even when private option is specified. Seems like a issue with the Spotify API. [Link](https://community.spotify.com/t5/Spotify-for-Developers/Api-to-create-a-private-playlist-doesn-t-work/td-p/5407807).
