use rspotify::{scopes, AuthCodeSpotify, Config, Credentials, OAuth};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to x playlist builder!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn init_spotify() -> AuthCodeSpotify {
    let config = Config {
        ..Default::default()
    };

    let oauth = OAuth {
        scopes: scopes!(
            "user-read-private",
            "user-read-email",
            "user-library-read",
            "user-library-modify"
        ),
        redirect_uri: "http://localhost:8000/callback".to_owned(),
        ..Default::default()
    };

    // Replacing client_id and client_secret with yours.
    let creds = Credentials::from_env();

    AuthCodeSpotify::with_config(creds.unwrap(), oauth, config)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
