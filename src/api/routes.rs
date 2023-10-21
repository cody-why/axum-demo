/*
 * @Author: plucky
 * @Date: 2023-10-18 16:20:11
 * @LastEditTime: 2023-10-18 23:24:00
 */

use axum::{http::Request, middleware, response::Response, routing::get_service, Router};
use tower_cookies::CookieManagerLayer;
use tower_http::{services::ServeDir, cors::{CorsLayer, Any}, trace::TraceLayer};
use tracing::info;

use super::user;

pub fn app() -> Router {
    Router::new()
        .nest("/api", user::routes())
        .layer(middleware::map_response(main_respone_mapper))
        // .layer(middleware::map_request(main_request_mapper))
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(Any))
        .fallback_service(routes_static())
}

// 静态文件路由
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new(".")))
}

// 中间件
async fn main_respone_mapper(res: Response) -> Response {
    info!("main_respone_mapper: {:?}", res);
    res
}

#[allow(unused)]
async fn main_request_mapper<B>(request: Request<B>) -> Request<B> {
    tracing::debug!("{:?}", request.uri());
    request
}
