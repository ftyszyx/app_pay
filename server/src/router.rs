use crate::handlers::{self, *};
use salvo::prelude::*;
use salvo::cors::{Cors, AllowOrigin, AllowHeaders};
use salvo::http::Method;
use crate::types::common::AppState;
use salvo_oapi::{OpenApi, SecurityScheme};
use salvo_oapi::security::{Http, HttpAuthScheme};


pub fn create_router(app_state: AppState) -> Service {
    let admin_routes = Router::with_path("/api/admin")
        .hoop(middleware::auth)
        .hoop(middleware::error_handler)
        .push(Router::with_path("me").get(handlers::auth::get_current_user))
        //users
        .push(Router::with_path("me/password").post(handlers::auth::change_password))
        .push(Router::with_path("users").post(handlers::user_handler::add))
        .push(Router::with_path("users/list").get(handlers::user_handler::get_list))
        .push(Router::with_path("users/{id}").get(handlers::user_handler::get_by_id))
        .push(Router::with_path("users/{id}").put(handlers::user_handler::update))
        .push(Router::with_path("users/{id}").delete(handlers::user_handler::delete))
        //apps
        .push(Router::with_path("apps").post(handlers::app_handler::add))
        .push(Router::with_path("apps/list").get(handlers::app_handler::get_list))
        .push(Router::with_path("apps/{id}").get(handlers::app_handler::get_by_id))
        .push(Router::with_path("apps/{id}").put(handlers::app_handler::update))
        .push(Router::with_path("apps/{id}").delete(handlers::app_handler::delete))
        //roles
        .push(Router::with_path("roles").post(handlers::role_handler::add))
        .push(Router::with_path("roles/list").get(handlers::role_handler::get_list))
        .push(Router::with_path("roles/{id}").get(handlers::role_handler::get_by_id))
        .push(Router::with_path("roles/{id}").put(handlers::role_handler::update))
        .push(Router::with_path("roles/{id}").delete(handlers::role_handler::delete))
        //products
        .push(Router::with_path("products").post(handlers::product_handler::add))
        .push(Router::with_path("products/list").get(handlers::product_handler::get_list))
        .push(Router::with_path("products/{id}").get(handlers::product_handler::get_by_id))
        .push(Router::with_path("products/{id}").put(handlers::product_handler::update))
        .push(Router::with_path("products/{id}").delete(handlers::product_handler::delete))
        //resources
        .push(Router::with_path("resources").post(handlers::resource_handler::add))
        .push(Router::with_path("resources/list").get(handlers::resource_handler::get_list))
        .push(Router::with_path("resources/{id}").get(handlers::resource_handler::get_by_id))
        .push(Router::with_path("resources/{id}").put(handlers::resource_handler::update))
        .push(Router::with_path("resources/{id}").delete(handlers::resource_handler::delete))
        //pay_methods
        .push(Router::with_path("pay_methods").post(handlers::pay_method_handler::add))
        .push(Router::with_path("pay_methods/list").get(handlers::pay_method_handler::get_list))
        .push(Router::with_path("pay_methods/{id}").get(handlers::pay_method_handler::get_by_id))
        .push(Router::with_path("pay_methods/{id}").put(handlers::pay_method_handler::update))
        .push(Router::with_path("pay_methods/{id}").delete(handlers::pay_method_handler::delete))
        //invite_records
        .push(Router::with_path("invite_records/list").get(handlers::invite_records_handler::get_list))
        .push(Router::with_path("invite_records/{id}").get(handlers::invite_records_handler::get_by_id))
        .push(Router::with_path("invite_records/{id}").put(handlers::invite_records_handler::update))
        .push(Router::with_path("invite_records/{id}").delete(handlers::invite_records_handler::delete))
        .push(Router::with_path("invite_records").post(handlers::invite_records_handler::add))
        //reg_codes
        .push(Router::with_path("reg_codes").post(handlers::reg_codes_handler::add))
        .push(Router::with_path("reg_codes/list").get(handlers::reg_codes_handler::get_list))
        .push(Router::with_path("reg_codes/{id}").get(handlers::reg_codes_handler::get_by_id))
        .push(Router::with_path("reg_codes/{id}").put(handlers::reg_codes_handler::update))
        .push(Router::with_path("reg_codes/{id}").delete(handlers::reg_codes_handler::delete))
        //orders
        .push(Router::with_path("orders/list").get(handlers::orders_handler::get_list))
        .push(Router::with_path("orders/{id}").get(handlers::orders_handler::get_by_id))
        .push(Router::with_path("orders/{id}").put(handlers::orders_handler::update))
        .push(Router::with_path("orders/{id}").delete(handlers::orders_handler::delete))
        .push(Router::with_path("orders").post(handlers::orders_handler::add))
        //coupons
        .push(Router::with_path("coupons").post(handlers::coupons_handler::add))
        .push(Router::with_path("coupons/list").get(handlers::coupons_handler::get_list))
        .push(Router::with_path("coupons/{id}").get(handlers::coupons_handler::get_by_id))
        .push(Router::with_path("coupons/{id}").put(handlers::coupons_handler::update))
        .push(Router::with_path("coupons/{id}").delete(handlers::coupons_handler::delete))
        //storage/oss/sts
        .push(Router::with_path("storage/oss/sts").get(handlers::oss_handler::get_oss_sts))
        //permissions
        .push(Router::with_path("permissions/policies").post(handlers::casbin_handler::add_policy))
        .push(Router::with_path("permissions/policies").delete(handlers::casbin_handler::remove_policy))
        .push(Router::with_path("permissions/policies").get(handlers::casbin_handler::get_policies))
        .push(Router::with_path("permissions/roles").post(handlers::casbin_handler::add_role_for_user))
        .push(Router::with_path("permissions/roles").delete(handlers::casbin_handler::remove_role_for_user))
        .push(Router::with_path("permissions/roles").get(handlers::casbin_handler::get_roles))
        .push(Router::with_path("permissions/check").post(handlers::casbin_handler::check_permission))
        .push(Router::with_path("permissions/reload").post(handlers::casbin_handler::reload_policies))
        //devices
        .push(Router::with_path("devices/list").get(handlers::device_handler::get_list));

    let cors = Cors::new()
    .allow_origin(AllowOrigin::any())
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
    // .allow_headers(vec!["authorization","content-type"]).into_handler();
    .allow_headers(AllowHeaders::any()).into_handler();
    let router=Router::new()
        .hoop(affix_state::inject(app_state))
        // .push(Router::with_path("/api/register").post(handlers::auth::register))
        .push(Router::with_path("/api/login").post(handlers::auth::login))
        .push  (Router::with_path("/api/reg/validate").post(handlers::reg_codes_handler::validate_code))
        .push(Router::with_path("/api/reg/validate").post(handlers::reg_codes_handler::validate_code))
        .push(Router::with_path("/api/reg/validate").get(handlers::reg_codes_handler::validate_code_get))
        .push( admin_routes)
        .push(Router::with_path("/api/vuefinder/list").get(handlers::vuefinder_handler::list));
    //添加swagger-ui
    let doc=OpenApi::new("app_server_api", "1.0.0")
        .add_security_scheme("bearer", SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer).bearer_format("JWT")))
        .merge_router(&router);
    let router=router.unshift(doc.into_router("/api-doc/openapi.json"))
    .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"));
    let service=Service::new(router).hoop(cors).hoop(Logger::new());
    service
}
