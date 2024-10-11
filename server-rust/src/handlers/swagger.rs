use utoipa::OpenApi;
use crate::handlers::users::__path_get_users;
use crate::handlers::users::__path_update_user_name;
use crate::handlers::health::__path_health_checker_handler;
use crate::handlers::auth::__path_login;
use crate::dto::dto::LoginUserDto;
use crate::dto::dto::NameUpdateDto;
use crate::dto::dto::LoginPlatform;
use crate::handlers::auth::AuthBody;
use crate::result::ResultTo;
#[derive(Debug, OpenApi)]
#[openapi(
    paths(
        login,
        health_checker_handler,
        get_users,
        update_user_name
    ),
    components(schemas(
        LoginUserDto,
        NameUpdateDto,
        LoginPlatform,
        AuthBody,
    )),
    tags(
        (name = "Category", description = "Category operations"),
    )
)]
pub struct ApiDoc;
