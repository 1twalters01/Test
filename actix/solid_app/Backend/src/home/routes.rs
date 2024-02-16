use actix_web::{get, HttpResponse, Responder, Result, web};
use std::{fs, path::PathBuf};

#[get("/favicon.ico")]
async fn favicon() -> Result<HttpResponse> {
    let image_content = web::Bytes::from(fs::read("../Frontend/public/favicon.ico").unwrap());
    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(image_content))
}

#[get("/{param:.*?}")]
async fn index() -> Result<impl Responder> {
    let index_path: PathBuf = "../Frontend/dist/index.html".into(); 
    let index = web::Bytes::from(fs::read(index_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(index))
}

#[get("/script.jsx")]
async fn script() -> Result<impl Responder> {
    let script_path: PathBuf = "../Frontend/dist/app.bundle.js".into();
    let script_data = web::Bytes::from(fs::read(script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=utf-8")
        .body(script_data))
}

#[get("/src_App_jsx.app.bundle.js")]
async fn app() -> Result<impl Responder> {
    let script_path: PathBuf = "../Frontend/dist/src_App_jsx.app.bundle.js".into();
    let script_data = web::Bytes::from(fs::read(script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=utf-8")
        .body(script_data))
}

#[get("/src_NotFound_jsx.app.bundle.js")]
async fn not_found() -> Result<impl Responder> {
    let script_path: PathBuf = "../Frontend/dist/src_NotFound_jsx.app.bundle.js".into();
    let script_data = web::Bytes::from(fs::read(script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=utf-8")
        .body(script_data))
}

#[get("/src_About_jsx.app.bundle.js")]
async fn about() -> Result<impl Responder> {
    let script_path: PathBuf = "../Frontend/dist/src_About_jsx.app.bundle.js".into();
    let script_data = web::Bytes::from(fs::read(script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=utf-8")
        .body(script_data))
}

#[get("/src_App_module_css.app.bundle.js")]
async fn css() -> Result<impl Responder> {
    let script_path: PathBuf = "../Frontend/dist/src_App_module_css.app.bundle.js".into();
    let script_data = web::Bytes::from(fs::read(script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=utf-8")
        .body(script_data))
}

