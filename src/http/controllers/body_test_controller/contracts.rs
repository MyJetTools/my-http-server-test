use std::collections::HashMap;

use my_http_server_swagger::{MyHttpInput, MyHttpInputObjectStructure};
use serde::Deserialize;

#[derive(MyHttpInput)]
pub struct BodyModelHttpInput {
    #[http_body_raw(name = "body"; description = "Body")]
    pub body: BodyModelContract,
}

#[derive(Deserialize, Debug, MyHttpInputObjectStructure)]
pub struct BodyModelContract {
    pub field1: usize,
    pub field2: String,
    pub field3: BodySubModel,
}

#[derive(Deserialize, Debug, MyHttpInputObjectStructure)]
pub struct BodySubModel {
    pub field1: usize,
    pub field2: String,
}

#[derive(MyHttpInput)]
pub struct BodyModelStringRawHttpInput {
    #[http_body_raw(name = "body"; description = "Body")]
    pub body: String,
}

#[derive(MyHttpInput)]
pub struct BodyModelU8RawHttpInput {
    #[http_body_raw(name = "body"; description = "Body")]
    pub body: u8,
}

#[derive(MyHttpInput)]
pub struct BodyModelI32RawHttpInput {
    #[http_body_raw(name = "body"; description = "Body")]
    pub body: i32,
}

#[derive(MyHttpInput)]
pub struct BodyModelVecOfI32RawHttpInput {
    #[http_body_raw(name = "body"; description = "Body")]
    pub body: Vec<i32>,
}

#[derive(MyHttpInput)]
pub struct BodyModelVecOfStringRawHttpInput {
    #[http_body_raw(name = "body"; description = "Body")]
    pub body: Vec<String>,
}

#[derive(MyHttpInput)]
pub struct BodyModelVecOfObjectRawHttpInput {
    #[http_body_raw(name = "body"; description = "Body")]
    pub body: Vec<BodyModelContract>,
}

#[derive(MyHttpInput)]
pub struct BodyModelHashmapOfObjectRawHttpInput {
    #[http_body_raw(name = "body"; description = "Body")]
    pub body: HashMap<String, BodyModelContract>,
}
