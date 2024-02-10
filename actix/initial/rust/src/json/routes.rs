use actix_web::{web, get, post, Responder, Result};
use super::datatypes::*;

// Basic get request
#[get("/")]
async fn hello() -> Result<impl Responder> {
    Ok(web::Json("Hello world!"))
}


// get with 1 paramater
#[get("/get-single/{param_1}")]
async fn get_single(param_1: web::Path<String>) -> Result<impl Responder> {
    let obj = GetSingle {
        name: param_1.into_inner(),
    };
    Ok(web::Json(obj))
}


// get with 2 parameters
#[get("/get-double/{param_1}/{param_2}")]
async fn get_double(params: web::Path<(String, f64)>) -> Result<impl Responder> {
    let (param1, param2) = params.into_inner();
    let obj = GetDouble {
        name: param1.to_string(),
        age: param2,
    };

    Ok(web::Json(obj))
}


// basic post request
#[post("/echo")]
async fn echo(req_body: web::Json<Payload>) -> Result<impl Responder> {
    let res_body: Payload = Payload {
        field_1: String::from("Response"),
        amount: req_body.into_inner().amount * 2,
    };

    Ok(web::Json(res_body))
}


// post with 1 parameter
#[post("/post-single/{param}")]
async fn post_single(param: web::Path<String>, req_body: web::Json<Payload>) -> Result<impl Responder> {
    let res_body: Payload2 = Payload2 {
        field_1: String::from("Response"),
        amount: req_body.into_inner().amount * 2,
        param: param.into_inner(),
    };

    Ok(web::Json(res_body))
}


// post with 2 parameters
#[post("/post-double/{param_1}/{param_2}")]
async fn post_double(params: web::Path<(String, f64)>, req_body: web::Json<Payload>) -> Result<impl Responder> {
    let (param1, param2) = params.into_inner();
    let res_body: Payload3 = Payload3 {
        field_1: String::from("Response"),
        amount: req_body.into_inner().amount * 2,
        param_1: param1,
        param_2: param2,
    };

    Ok(web::Json(res_body))
}

