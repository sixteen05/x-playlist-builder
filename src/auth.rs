use rspotify::{
    scopes, AuthCodeSpotify, Credentials, OAuth,
};

pub fn init_spotify() -> AuthCodeSpotify {
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
    AuthCodeSpotify::new(creds.unwrap(), oauth)
}