use actix_web::{web, App, HttpResponse, HttpServer};


async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/health_check", web::get().to(health_check))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
