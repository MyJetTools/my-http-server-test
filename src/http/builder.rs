use std::sync::Arc;

use my_http_server_controllers::controllers::{
    ControllersAuthorization, ControllersMiddleware, RequiredClaims,
};

pub fn build_controllers() -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new(
        ControllersAuthorization::BearerAuthentication {
            global: false,
            global_claims: RequiredClaims::no_claims(),
        }
        .into(),
        None,
    );

    result.register_post_action(Arc::new(
        super::controllers::body_as_model_controller::PostAction,
    ));

    result.register_post_action(Arc::new(
        super::controllers::from_data_controller::TestFormDataAction::new(),
    ));

    result.register_post_action(Arc::new(
        super::controllers::body_test_controller::PostBodyModelAction::new(),
    ));

    result.register_post_action(Arc::new(
        super::controllers::body_test_controller::PostBodyStringRawModelAction::new(),
    ));

    result.register_post_action(Arc::new(
        super::controllers::body_test_controller::PostBodyU8RawModelAction::new(),
    ));

    result.register_post_action(Arc::new(
        super::controllers::body_test_controller::PostBodyI32RawModelAction::new(),
    ));

    result.register_post_action(Arc::new(
        super::controllers::body_test_controller::PostBodyVecOfI32RawModelAction::new(),
    ));

    result.register_post_action(Arc::new(
        super::controllers::body_test_controller::PostBodyVecOfStringRawModelAction::new(),
    ));

    result.register_post_action(Arc::new(
        super::controllers::body_test_controller::PostBodyVecOfObjectRawModelAction::new(),
    ));

    result.register_post_action(Arc::new(
        super::controllers::body_test_controller::PostBodyHashmapOfObjectRawModelAction::new(),
    ));

    result.register_get_action(Arc::new(
        super::controllers::test_path_controller::TestPathAction::new(),
    ));

    result
}
