use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::types::common::{AppState, Claims};
use http_body_util::BodyExt;

pub async fn auth(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });

    let token = token.ok_or(StatusCode::UNAUTHORIZED)?;
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(state.config.jwt.secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    req.extensions_mut().insert(decoded.claims);
    Ok(next.run(req).await)
}

pub async fn error_handler(
    req: Request<Body>,
    next: Next,
) -> Response {
    let response = next.run(req).await;
    let status=response.status();
    if status!=StatusCode::OK{
        let (parts, body) = response.into_parts();
        let bytes = body.collect().await.unwrap().to_bytes();
        let body_str=String::from_utf8_lossy(&bytes);
        tracing::error!("Response status: {} body: {}", status, body_str);
        Response::from_parts(parts, Body::from(bytes))
    }
    else {
        response
    }
}