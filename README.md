# x-playlist-builder

## Current State
WIP :construction:

A tool to create playlist from your liked songs in Spotify. Currently couple condition supported -
- 'old-hindi' hindi songs released before 1990
- 'artist' songs by artist name.

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
- Run `cargo run`.
- Visit `http://localhost:8080/liked/create-update-playlist/condition/artist/arijit` it will then ask you can to authorize this app in spotify. Then follow the instruction in the terminal.
- After the above call is executed you would have a public playlist in spotify with songs by artist `Arijit` if they existed in your liked songs list.
- Replace `arijit` with any other artist name that should create a new playlist for that artist.

## Known Issues
- Currently, the playlist created is public even when private option is specified. Seems like a issue with the Spotify API. [Link](https://community.spotify.com/t5/Spotify-for-Developers/Api-to-create-a-private-playlist-doesn-t-work/td-p/5407807).