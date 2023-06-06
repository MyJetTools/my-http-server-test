use my_http_server::HttpFailResult;
use my_http_server_swagger::{MyHttpInput, MyHttpInputObjectStructure, MyHttpObjectStructure};
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

#[derive(MyHttpInput)]
pub struct TestStruct {
    #[http_query(name = "dd"; description = "DateTime")]
    pub dd: String,

    #[http_query(description = "DateTime")]
    pub password: String,
    #[http_body(name = "dtFrom"; description = "Date time form")]
    pub dt_from: Option<String>,
    #[http_body(name = "dtTo"; description = "Date time form")]
    pub dt_to: String,
    #[http_body(name = "sub"; description = "Date time form")]
    pub sub_structure: SubStructure,
}

#[derive(Serialize, Deserialize, Debug, MyHttpInputObjectStructure)]
pub struct SubStructure {
    pub field1: String,
    pub field2: String,
}
