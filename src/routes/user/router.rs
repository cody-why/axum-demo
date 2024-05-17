/*
 * @Author: plucky
 * @Date: 2023-10-18 21:22:59
 * @LastEditTime: 2023-12-12 21:19:11
 */


use axum::{response::IntoResponse, Json, Router, routing::{post, get}, http::HeaderMap};
use tower_cookies::{Cookies, Cookie};
use tracing::info;
use crate::{Result, Error, routes::{auth::AUTH_TOKEN, auth::jwt::Claims}, RespVO};

use super::dto::*;



/// routes
pub fn routes() -> Router {
    Router::new()
        .route("/logout", post(logout))
        .route("/check", get(check))
        // .route_layer(middleware::from_fn(auth))
        .route("/login", post(login))
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "successfully", body = RespVO),
        (status = 401, description = "无访问权限或其他错误", body = RespVO),
    ),
    security(
        ("api_key" = [])
    ),
)]
/// login
pub(crate) async fn login(cookies: Cookies, Json(payload): Json<LoginPayload>) -> Result<impl IntoResponse> {
    info!("login payload: {:?}", payload);
    if payload.password != "123456" {
        return Err(Error::LoginError("password error".into()));
    }
    // 创建一个24小时的token
    let claims  =  Claims::new(1, payload.username, 3600*24);
    let token = claims.encode().unwrap();
    
    cookies.add(Cookie::new(AUTH_TOKEN, token.clone()));
    
    let mut headers = HeaderMap::new();
    headers.append(AUTH_TOKEN, token.parse().unwrap());
    Ok((headers, RespVO::success("login success")))
}

async fn logout(cookies: Cookies) -> Result<impl IntoResponse> {
    cookies.remove(Cookie::from(AUTH_TOKEN));
    Ok(RespVO::success("logout success"))
}

async fn check(_claims: Claims) -> Result<impl IntoResponse> {
    
    Ok(RespVO::success("check success"))
}
