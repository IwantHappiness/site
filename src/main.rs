use actix_files::{Files, NamedFile};
use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
	NamedFile::open_async("./dist/index.html").await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	println!("ваш сайт: http://localhost:8080");

	HttpServer::new(|| {
		App::new().service(index).service(Files::new("/", "./dist"))
	})
	.bind(("0.0.0.0", 8080))?
	.run()
	.await
}
