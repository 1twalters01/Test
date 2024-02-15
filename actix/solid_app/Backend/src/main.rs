mod routes;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::routes::script)
            .service(routes::routes::css)
            .service(routes::routes::about)
            .service(routes::routes::app)
            .service(routes::routes::not_found)
            .service(routes::routes::index)
            
            .service(
                web::scope("/api")
                .service(account::routes::login)
                .service(account::routes::logout)
                .service(account::register)
            )
    })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
