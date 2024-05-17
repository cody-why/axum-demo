/*
 * @Author: plucky
 * @Date: 2023-10-18 16:20:11
 * @LastEditTime: 2023-12-12 21:19:45
 */

use std::time::Duration;

use axum::{http::Request, response::{Response, IntoResponse}, routing::{get_service, get}, Router};
use tower_cookies::CookieManagerLayer;
use tower_http::{services::ServeDir, cors::{CorsLayer, Any}, trace::TraceLayer};
use tracing::{info, Span, debug};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use super::{user, auth, api_doc::ApiDoc};

pub fn app() -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .on_request(|request: &Request<_>, _span: &Span| {
            debug!("on_request: {} {}, {request:?}", request.method(), request.uri().path());
        })
        .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
            debug!("on_response: in {:?}ms, {:?}", latency.as_millis(), response);
        });

    Router::new()
        .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", ApiDoc::openapi()).path("/apidoc"))
        .route("/", get(index))
        .merge(auth::routes())
        .nest("/api", user::routes())
        .layer(CookieManagerLayer::new())
        // .layer(axum::middleware::map_response(main_respone_mapper))
        // .layer(middleware::map_request(main_request_mapper))
        .layer(trace_layer)
        .layer(CorsLayer::new().allow_origin(Any))

        

        .fallback_service(routes_static())
        
}

async fn index() -> impl IntoResponse {
    "Hello, World!"
}

// 静态文件路由
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new(".")))
}

// 中间件
#[allow(unused)]
async fn main_respone_mapper(res: Response) -> Response {
    info!("main_respone: {:?}", res);
    res
}

#[allow(unused)]
async fn main_request_mapper<B>(request: Request<B>) -> Request<B> {
    debug!("main_request: {:?} {:?} {:?}", request.method(), request.uri(), request.headers());
    request
}


