use my_http_server::types::FileContent;
use my_http_server_swagger::{MyHttpInput, MyHttpObjectStructure};
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

#[derive(MyHttpInput)]
pub struct TestStruct {
    #[http_query(name = "dd"; description = "DateTime")]
    pub dd: String,

    #[http_query(description = "DateTime")]
    pub password: String,
    #[http_form_data(name = "dtFrom"; description = "Date time form")]
    pub dt_from: Option<String>,

    #[http_form_data(name = "dtTo"; description = "Date time form")]
    pub dt_to: String,

    #[http_form_data(name = "document"; description = "Upload document")]
    pub file: FileContent,

    #[http_form_data(name = "documentOpt"; description = "Upload document")]
    pub file_opt: Option<FileContent>,
}

#[test]
fn test() {}
