use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run() -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(("localhost", 8000))?
        .run();

    Ok(server)
}
