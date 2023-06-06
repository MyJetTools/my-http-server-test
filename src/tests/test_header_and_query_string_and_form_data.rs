use my_http_server::types::FileContent;
use my_http_server_swagger::MyHttpInput;

#[derive(MyHttpInput)]
pub struct TestStruct {
    #[http_query(name = "dd"; description = "DateTime")]
    pub dd: String,

    #[http_query(description = "DateTime")]
    pub password: String,
    #[http_form_data(name = "dtFrom"; description = "Date time form")]
    pub dt_from: Option<String>,
}

#[derive(MyHttpInput)]
pub struct TestStruct2 {
    #[http_query(name = "dd"; description = "DateTime")]
    pub dd: String,

    #[http_query(description = "DateTime")]
    pub password: String,
    #[http_form_data(name = "dtFrom"; description = "Date time form")]
    pub dt_from: String,
}

#[derive(MyHttpInput)]
pub struct TestStruct3 {
    #[http_query(name = "dd"; description = "DateTime")]
    pub dd: String,
    #[http_query(description = "DateTime")]
    pub password: String,

    #[http_query(description = "Bool Examplse")]
    pub my_bool: bool,
    #[http_form_data(name = "uploadFile"; description = "File to upload")]
    pub dt_from: FileContent,
}

#[test]
fn test() {}
