use actix_web::{get, post, HttpRequest, HttpResponse, Responder, Result, web};
use actix_files::NamedFile;
use std::path::PathBuf;

// Basic get request
#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("index.html")))
}

#[get("/favicon.ico")]
// pub async fn favicon(req: HttpRequest) -> Result<NamedFile> {
async fn favicon(req: HttpRequest) -> Result<HttpResponse> {
    // let path: PathBuf = req.match_info().query("./favicon.ico").parse().unwrap();
    // let file = NamedFile::open(path)?;
    // Ok(file)

    // Ok(NamedFile::open("favicon.png")?)

    let image_content = web::Bytes::from(std::fs::read("favicon.png")?);
    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(image_content))
}

#[get("/styles.css")]
async fn style() -> Result<impl Responder> {
    Ok(HttpResponse::Ok()
      .content_type("text/css; charset=utf-8")
      .body(include_str!("styles.css")))
}

#[get("/script.js")]
async fn script() -> Result<impl Responder> {
    Ok(HttpResponse::Ok()
    .content_type("text/javascript; charset=utf-8")
    .body(include_str!("script.js")))
}

// #[post("/form")]
// async fn form(req_body: web::Json<Payload>) -> impl Responder {
    // let res_body: Payload = Payload {
        // field_1: String::from("Response"),
        // amount: req_body.into_inner().amount * 2,
    // };

    // web::Json(res_body)
// }
