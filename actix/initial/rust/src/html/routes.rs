use actix_web::{get, post, HttpRequest, HttpResponse, Responder, Result, web};
use serde::{Deserialize, Serialize};
use async_stream::{try_stream, __private::AsyncStream};
use std::fs;
use std::io::Read;
use bytes::Bytes;

// Basic get request for html
#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("index.html")))
}

// Basic get request for css
#[get("/styles.css")]
async fn style() -> Result<impl Responder> {
    Ok(HttpResponse::Ok()
      .content_type("text/css; charset=utf-8")
      .body(include_str!("styles.css")))
}

// Basic get request for js
#[get("/script.js")]
async fn script() -> Result<impl Responder> {
    Ok(HttpResponse::Ok()
    .content_type("text/javascript; charset=utf-8")
    .body(include_str!("script.js")))
}


// Get favicon
#[get("/favicon.ico")]
// pub async fn favicon(req: HttpRequest) -> Result<NamedFile> {
async fn favicon() -> Result<HttpResponse> {
    // if running from rust and not rust/src
    // println!("The current directory is {}", std::env::current_dir()?.display());
    let image_content = web::Bytes::from(std::fs::read("src/html/favicon.ico").unwrap());
    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(image_content))
}



// Get large image
#[get("/large-image")]
// pub async fn favicon(req: HttpRequest) -> Result<NamedFile> {
async fn large_image() -> Result<HttpResponse> {
    // if running from rust and not rust/src
    // println!("The current directory is {}", std::env::current_dir()?.display());
    // let image_content = web::Bytes::from(std::fs::read("src/html/favicon.ico").unwrap());
    println!("working");
    let stream: AsyncStream<Result<Bytes>, _> = try_stream! {
        // let mut file = fs::File::open("src/html/large-image.jpg").unwrap();
        let mut file = fs::File::open("src/html/favicon.ico").unwrap();
        // let mut buffer: Vec<u8> = Vec::new();
        let mut buffer = [0; 40960];
        loop {
            let n = file.read(&mut buffer[..])?;
            if n == 0 {
                break;
            }
            yield bytes::Bytes::copy_from_slice(&buffer[n..]);
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("image/jpg")
        .streaming(stream))
}



// Get small video
#[get("/small-video")]
// pub async fn favicon(req: HttpRequest) -> Result<NamedFile> {
async fn small_video() -> Result<HttpResponse> {
    // if running from rust and not rust/src
    // println!("The current directory is {}", std::env::current_dir()?.display());
    let image_content = web::Bytes::from(std::fs::read("src/html/favicon.ico").unwrap());
    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(image_content))
}



// Get large video and stream it
#[get("/large-video")]
// pub async fn favicon(req: HttpRequest) -> Result<NamedFile> {
async fn large_video() -> Result<HttpResponse> {
    let image_content = web::Bytes::from(std::fs::read("src/html/favicon.ico").unwrap());
    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(image_content))
}



#[derive(Debug, Deserialize, Serialize)]
pub struct GetSingle {
    pub name: String,
}


// get with 1 paramater for html
#[get("/get-single/{param_1}")]
async fn get_single(param_1: web::Path<String>) -> Result<impl Responder> {
    let obj = GetSingle {
        name: param_1.into_inner(),
    };
    println!("obj: {:?}", obj);

    // Ok(HttpResponse::Ok()
    //     .content_type("text/html; charset=utf-8")
    //     .body(include_str!("index.html")))

    // println!("html content: {:?}", html_content);

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("get-single.html")
            // .replace("``", param_1.into_inner().as_str())
            .as_bytes()))

}






// Basic post request using a form
// #[post("/form")]
// async fn form(req_body: web::Json<Payload>) -> impl Responder {
    // let res_body: Payload = Payload {
        // field_1: String::from("Response"),
        // amount: req_body.into_inner().amount * 2,
    // };

    // web::Json(res_body)
// }


// Basic post request using a form, and also with 2 parameters
