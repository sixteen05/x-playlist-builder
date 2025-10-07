use rspotify::{prelude::*, scopes, AuthCodeSpotify, Config, Credentials, OAuth};

pub struct SpotifyAuth {
    pub client: AuthCodeSpotify,
}

impl SpotifyAuth {
    pub async fn new() -> Self {
        let oauth = OAuth {
            scopes: scopes!(
                "user-read-private",
                "user-read-email",
                "user-library-read",
                "user-library-modify",
                "playlist-modify-private",
                "playlist-read-private"
            ),
            redirect_uri: "http://localhost:8080/callback".to_owned(),
            ..Default::default()
        };

        let creds = Credentials::from_env();

        // Enable token caching and auto-refresh
        let config = Config {
            token_cached: true,
            token_refreshing: true,
            ..Default::default()
        };

        let mut client = AuthCodeSpotify::with_config(creds.unwrap(), oauth, config);

        // Try to read cached token first, only prompt if not available or expired
        match client.read_token_cache(false).await {
            Ok(Some(_)) => {
                println!("Using cached token");
            }
            _ => {
                println!("No valid cached token found, requesting authorization...");
                let url = client.get_authorize_url(false).unwrap();
                client.prompt_for_token(&url).await.unwrap();
                // Token is automatically cached by rspotify
            }
        }

        Self { client }
    }
}
