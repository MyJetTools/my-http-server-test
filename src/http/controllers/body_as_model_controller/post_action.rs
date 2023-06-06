use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;

/*
#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/bodymodel/v1",
    summary: "Test of body as model",
    description: "Test of body as model",
    controller: "Test",
    input_data: "PostBodyModel",
    result:[
        {status_code: 200, description: "Ok response", model: "DoneResult"},
    ]
)]
pub struct PostAction {}

impl PostAction {
    pub fn new() -> Self {
        Self {}
    }
}

*/

pub struct PostAction;

impl my_http_server_controllers::controllers::actions::PostAction for PostAction {
    fn get_route(&self) -> &str {
        "/api/bodymodel/v1"
    }
    fn get_model_routes(&self) -> Option<Vec<&'static str>> {
        PostBodyModel::get_model_routes()
    }
}
impl my_http_server_controllers::controllers::actions::GetDescription for PostAction {
    fn get_description(
        &self,
    ) -> Option<my_http_server_controllers::controllers::documentation::HttpActionDescription> {
        use my_http_server_controllers::controllers::documentation::*;
        HttpActionDescription {
            controller_name: "Test",
            summary: "Test of body as model",
            description: "Test of body as model",
            should_be_authorized: ShouldBeAuthorized::UseGlobal,
            input_params: PostBodyModel::get_input_params().into(),
            results: vec![out_results::HttpResult {
                nullable: false,
                description: "Ok response".to_string(),
                http_code: 200,
                data_type: DoneResult::get_http_data_structure().into_http_data_type_object(),
            }],
        }
        .into()
    }
}
impl my_http_server_controllers::controllers::actions::HandleHttpRequest for PostAction {
    #[allow(
        clippy::async_yields_async,
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn handle_request<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        http_route: &'life1 my_http_server_controllers::controllers::HttpRoute,
        ctx: &'life2 mut my_http_server::HttpContext,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = Result<my_http_server::HttpOkResult, my_http_server::HttpFailResult>,
                > + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                Result<my_http_server::HttpOkResult, my_http_server::HttpFailResult>,
            > {
                return __ret;
            }
            let __self = self;
            let __ret: Result<my_http_server::HttpOkResult, my_http_server::HttpFailResult> = {
                let input_data = PostBodyModel::parse_http_input(http_route, ctx).await?;
                handle_request(__self, input_data, ctx).await
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}

async fn handle_request(
    _action: &PostAction,
    input_data: PostBodyModel,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    println!("InputData: {:?}", input_data);
    return HttpOutput::Empty.into_ok_result(true).into();
}
