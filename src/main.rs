use actix_files::{Files, NamedFile};
use actix_web::{get, App, HttpServer, Responder};
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("./dist/index.html").await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap();

    HttpServer::new(|| App::new().service(index).service(Files::new("/", "./dist")))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
