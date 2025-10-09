use salvo::test::TestClient;
mod helpers;

#[tokio::test]
async fn test_get_oss_sts_success() {
	let app = helpers::create_test_app().await;
	let token = helpers::create_test_user_and_login(&app).await;
	let response = TestClient::get(helpers::get_url("/api/admin/storage/oss/sts"))
		.add_header("authorization", format!("Bearer {}", token), true)
		.send(&app)
		.await;
	let bodyjson = helpers::print_response_body_get_json(response, "get_oss_sts_success").await;
	let success = bodyjson["success"].as_bool().unwrap_or(false);
	assert!(success);
	let data = &bodyjson["data"];
	assert!(data["access_key_id"].is_string());
	assert!(data["access_key_secret"].is_string());
	assert!(data["security_token"].is_string());
	assert!(data["expiration"].is_string());
	assert!(!data["access_key_id"].as_str().unwrap().is_empty());
	assert!(!data["access_key_secret"].as_str().unwrap().is_empty());
	assert!(!data["security_token"].as_str().unwrap().is_empty());
	assert!(!data["expiration"].as_str().unwrap().is_empty());
}
