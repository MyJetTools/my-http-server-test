use my_http_server::types::RawData;
use my_http_server_swagger::{MyHttpInput, MyHttpInputObjectStructure};
use serde::{Deserialize, Serialize};

#[derive(MyHttpInput)]
pub struct TestStruct {
    #[http_query(name = "dd"; description = "DateTime")]
    pub dd: String,

    #[http_query(description = "DateTime")]
    pub password: String,
    #[http_body(name = "dtFrom"; description = "Date time form")]
    pub dt_from_opt: Option<String>,
}

#[derive(MyHttpInput)]
pub struct TestStruct2 {
    #[http_query(name = "dd"; description = "DateTime")]
    pub dd: String,

    #[http_query(description = "DateTime")]
    pub password: String,
    #[http_body(name = "dtFrom"; description = "Date time form")]
    pub dt_from: String,
}

#[derive(MyHttpInput)]
pub struct TestStruct3 {
    #[http_query(name = "dd"; description = "DateTime")]
    pub dd: String,

    #[http_query(description = "DateTime")]
    pub password: String,
    #[http_body_raw(name = "dtFrom"; description = "Date time form")]
    pub dt_from: RawData,
}

#[derive(MyHttpInput)]
pub struct TestWithSubStructure {
    #[http_query(name = "dd"; description = "DateTime")]
    pub dd: String,

    #[http_query(description = "DateTime")]
    pub password: String,
    #[http_body(name = "dtFrom"; description = "Date time form")]
    pub dt_from: SubStructure,
}

#[derive(Serialize, Deserialize, Debug, MyHttpInputObjectStructure)]
pub struct SubStructure {
    pub field1: String,
    pub field2: String,
}
