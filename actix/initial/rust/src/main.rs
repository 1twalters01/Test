mod json;
mod html;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .service(Files::new("/favicon.png", "./favicon.png"))
            // .route("/favicon.ico", web::get().to(html::routes::favicon))
            .service(html::routes::favicon)
            // .service(html::routes::test)
            .service(
                web::scope("/json")
                .service(json::routes::hello)
                .service(json::routes::echo)
                .service(json::routes::get_single)
                .service(json::routes::get_double)
                .service(json::routes::post_single)
                .service(json::routes::post_double)
            )
            .service(
                web::scope("/html")
                .service(html::routes::index)
                .service(html::routes::style)
                .service(html::routes::script)
                .service(html::routes::large_image)
                .service(html::routes::small_video)
                .service(html::routes::large_video)
                .service(html::routes::get_single)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
