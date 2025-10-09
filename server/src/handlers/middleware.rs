use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::types::common::{AppState, Claims};
use salvo::prelude::*;

#[handler]
pub async fn auth(
    req: &mut Request,
    depot:&mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) -> Result<(), StatusCode>{
    let state = depot.obtain::<AppState>().unwrap();
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
    depot.inject(decoded.claims);
    // ctrl.call_next(req, depot, res).await;
    Ok(())
}

#[handler]
pub async fn error_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    // 先放行到下游处理
    ctrl.call_next(req, depot, res).await;
    if let Some(code) = res.status_code{
        if code.as_u16() >= 400 {
            tracing::error!("Response status: {}", code);
        }
    }
    ctrl.call_next(req, depot, res).await;
}