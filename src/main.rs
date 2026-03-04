use actix_files::{Files, NamedFile};
use actix_web::{get, App, HttpServer, Responder};
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("./dist/index.html").await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|e| e.parse().ok())
        .unwrap_or(8080);

    HttpServer::new(|| App::new().service(index).service(Files::new("/", "./dist")))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
