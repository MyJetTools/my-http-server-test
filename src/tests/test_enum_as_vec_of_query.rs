use my_http_server_swagger::{MyHttpInput, MyHttpIntegerEnum};
use serde_repr::*;

#[derive(Debug, MyHttpIntegerEnum, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum OrderStatus {
    #[default]
    #[http_enum_case(id: 0; description:"Paid")]
    Paid = 0,
    #[http_enum_case(id: 1; description:"Pending")]
    Pending = 1,
    #[http_enum_case(id: 2; description:"Unpaid")]
    Unpaid = 2,
    #[http_enum_case(id: 3; description:"Cancelled")]
    Cancelled = 3,
}

#[derive(Debug, MyHttpInput)]
pub struct GetOrdersHttpInputData {
    #[http_query(name = "limit", description = "limit")]
    pub limit: u64,

    #[http_query(name = "offset", description = "offset")]
    pub offset: u64,

    #[http_query(name = "statuses", description = "statuses")]
    pub statuses: Vec<OrderStatus>,
}
