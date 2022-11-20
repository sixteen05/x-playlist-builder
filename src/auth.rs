use rspotify::{scopes, AuthCodeSpotify, Credentials, OAuth, prelude::*,};

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
        let mut client = AuthCodeSpotify::new(creds.unwrap(), oauth);
        let url = client.get_authorize_url(false).unwrap();
        client.prompt_for_token(&url).await.unwrap();
        Self {
            client: client,
        }
    }
}
