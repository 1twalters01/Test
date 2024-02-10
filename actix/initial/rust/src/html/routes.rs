use actix_web::{get, post, HttpRequest, HttpResponse, Responder, Result, web};
use actix_files::NamedFile;
use std::path::PathBuf;

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


// get with 1 paramater for html
#[get("/get-single/{param_1}")]
async fn get_single(param_1: web::Path<String>) -> Result<impl Responder> {
    let obj = GetSingle {
        name: param_1.into_inner(),
    };
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("index.html")))
}


// Get request with 2 parameters
#[get("/get-double/{param_1}/{param_2}")]
async fn get_double(params: web::Path<(String, f64)>) -> Result<HttpResponse> {
    let (param1, param2) = params.into_inner();
    let obj = GetDouble {
        name: param1.to_string(),
        age: param2,
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("index.html")))
}


// Get large image


// Get small video



// Get large video and stream it





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
