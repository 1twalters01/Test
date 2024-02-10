use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetSingle {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetDouble {
    pub name: String,
    pub age: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Payload {
    pub field_1: String,
    pub amount: i64,
}

#[derive(Serialize)]
pub struct Payload2 {
    pub field_1: String,
    pub amount: i64,
    pub param: String,
}

#[derive(Serialize)]
pub struct Payload3 {
    pub field_1: String,
    pub amount: i64,
    pub param_1: String,
    pub param_2: f64,
}

