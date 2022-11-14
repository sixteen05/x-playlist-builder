use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures::stream::TryStreamExt;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AuthCallbackRequest {
    code: String,
}

#[get("/")]
async fn _index() -> impl Responder {
    let spotify = init_spotify();
    let auth_url = spotify.get_authorize_url(true).unwrap();
    return auth_url;
}

#[get("/callback")]
async fn _callback(info: web::Query<AuthCallbackRequest>) -> impl Responder {
    let mut spotify = init_spotify();
    let code = &info.code;
    spotify.request_token(code).await.unwrap();
    return HttpResponse::Ok().body("Got the code & access token!");
}

#[get("/liked/songs")]
async fn liked_songs() -> impl Responder {
    let mut spotify = init_spotify();
    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();
    let stream = spotify.current_user_saved_tracks(None);
    println!("\nItems (concurrent):");
    stream
        .try_for_each_concurrent(10, |item| async move {
            let r = item.track.album.release_date.unwrap();
            let release_year = r.split("-").next().unwrap();
            let year_val = release_year.parse::<i32>().unwrap();
            if year_val < 1990 {
                println!("* {}, Year - {}", item.track.name, release_year);
            }
            Ok(())
        })
        .await
        .unwrap();

    return HttpResponse::Ok().body("Got liked songs!");
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn init_spotify() -> AuthCodeSpotify {
    let oauth = OAuth {
        scopes: scopes!(
            "user-read-private",
            "user-read-email",
            "user-library-read",
            "user-library-modify"
        ),
        redirect_uri: "http://localhost:8080/callback".to_owned(),
        ..Default::default()
    };

    let creds = Credentials::from_env();
    AuthCodeSpotify::new(creds.unwrap(), oauth)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .service(index)
            .service(echo)
            // .service(callback)
            .service(liked_songs)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
