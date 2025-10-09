use salvo::prelude::*;
use salvo::test::TestClient;
mod helpers;

#[tokio::test]
async fn test_get_pay_methods_list() {
	let app = helpers::create_test_app().await;
	let token = helpers::create_test_user_and_login(&app).await;
	let response = TestClient::get(helpers::get_url("/api/admin/pay_methods/list?page=1&page_size=10"))
		.add_header("authorization", format!("Bearer {}", token), true)
		.send(&app)
		.await;
	assert_eq!(response.status_code, Some(StatusCode::OK));
	let json = helpers::print_response_body_get_json(response, "get_pay_methods_list").await;
	assert!(json["success"].as_bool().unwrap());
	assert!(json["data"]["list"].is_array());
	assert!(json["data"]["total"].is_number());
}

#[tokio::test]
async fn test_pay_methods_pagination() {
	let app = helpers::create_test_app().await;
	let token = helpers::create_test_user_and_login(&app).await;
	let test_cases = vec![
		helpers::get_url("/api/admin/pay_methods/list?page=1&page_size=5"),
		helpers::get_url("/api/admin/pay_methods/list"),
	];
	for url in test_cases {
		let response = TestClient::get(url)
			.add_header("authorization", format!("Bearer {}", token), true)
			.send(&app)
			.await;
		assert_eq!(response.status_code, Some(StatusCode::OK));
	}
}
