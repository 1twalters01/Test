use actix_web::http::header::ContentLength;
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



use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

// Get large video and stream it
#[get("/large-video")]
// pub async fn favicon(req: HttpRequest) -> Result<NamedFile> {
async fn large_video(req: HttpRequest) -> Result<HttpResponse> {
    let path = "../../../../../Videos/Melissa.mp4";
    let file = match File::open(path).await {
        Ok(file) => file,
        Err(_) => return Ok(HttpResponse::NotFound().finish())
    };

    // Get total length of file
    let metadata = match file.metadata().await {
        Ok(metadata) => metadata,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let total_size = metadata.len();

    // Handle Range header if present
    let range_header = req.headers().get("Range").and_then(|h| h.to_str().ok());
    let (start, end) = if let Some(range) = range_header {
        let bytes_str = range.strip_prefix("bytes=").unwrap_or("");
        let mut parts = bytes_str.split('-');
        let start: u64 = parts.next().unwrap().parse().unwrap_or(0);
        let end: u64 = parts.next().unwrap_or("").parse().unwrap_or(total_size - 1);
        (start, end)
    } else {
        (0, total_size - 1)
    };

    let chunk_size = (end - start + 1) as usize;
    let mut file = file;
    if let Err(_) = file.seek(tokio::io::SeekFrom::Start(start)).await {
        return Ok(HttpResponse::InternalServerError().finish());
    }

    // Create a stream to read the file in chunks
    let stream = futures_util::stream::unfold(file, move |mut file| async move {
        let mut buffer = vec![0; 8192];
        match file.read(&mut buffer).await {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    None
                } else {
                    Some((Ok(Bytes::from(buffer[..bytes_read].to_vec())), file))
                }
            },
            Err(e) => Some((Err(e), file))
        }
    });

    Ok(HttpResponse::PartialContent()
        .insert_header(("Content-Range", format!("bytes {}-{}/{}", start, end, total_size)))
        .insert_header(ContentLength(chunk_size))
        .streaming(stream))

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
