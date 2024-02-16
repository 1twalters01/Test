use actix_web::{get, HttpResponse, Responder, Result, web};
use std::{fs, path::PathBuf};

#[get("/src_NotFound_jsx.app.bundle.js")]
async fn register() -> Result<impl Responder> {
    let script_path: PathBuf = "../Frontend/dist/src_NotFound_jsx.app.bundle.js".into();
    let script_data = web::Bytes::from(fs::read(script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=utf-8")
        .body(script_data))
}

#[get("/src_NotFound_jsx.app.bundle.js")]
async fn login() -> Result<impl Responder> {
    let script_path: PathBuf = "../Frontend/dist/src_NotFound_jsx.app.bundle.js".into();
    let script_data = web::Bytes::from(fs::read(script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=utf-8")
        .body(script_data))
}

#[get("/src_NotFound_jsx.app.bundle.js")]
async fn logout() -> Result<impl Responder> {
    let script_path: PathBuf = "../Frontend/dist/src_NotFound_jsx.app.bundle.js".into();
    let script_data = web::Bytes::from(fs::read(script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=utf-8")
        .body(script_data))
}
