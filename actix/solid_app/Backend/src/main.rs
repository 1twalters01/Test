mod home;
mod accounts;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home::routes::script)
            .service(home::routes::css)
            .service(home::routes::about)
            .service(home::routes::app)
            .service(home::routes::not_found)
            .service(home::routes::index)
            
            .service(
                web::scope("/api")
                .service(accounts::routes::login)
                .service(accounts::routes::logout)
                .service(accounts::routes::register)
            )
    })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
