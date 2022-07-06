/*** 
 * @Author: plucky
 * @Date: 2022-06-27 20:06:13
 * @LastEditTime: 2022-07-04 12:07:55
 * @Description: 
 */

use std::sync::Arc;

use axum::handler::Handler;
use axum::http::{HeaderValue, StatusCode, Uri};

use axum::routing::{get,self, post};
use axum::{Extension, Router};
use tower_http::trace::TraceLayer;

use crate::app::controllers::{shortlink_controller, user_controller};
use crate::app::state::State;

use tower_http::cors::CorsLayer;

pub fn app(state: State) -> Router {
    //let state = Arc::new(state::State { count: 0 });
    Router::new()
        .route("/", get(|| async { "welcome !!!" }))
        .nest("/api", short_links().merge(user_links()))
        
        .layer(
            // 用塔式增加中间件
            tower::ServiceBuilder::new()
                //.layer(Extension(pool))
                //.layer(Extension(Arc::new(Mutex::new(redis_conn))))
                
                .layer(Extension(Arc::new(state)))
                // 添加跟踪层,需要开启Debug日志
                .layer(TraceLayer::new_for_http())
                // 添加跨域层
                .layer(CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()))
                /* .allow_methods([Method::GET]) )*/
        )
        // 全局的404,等价于Handler::into_service(fallback_404)
        .fallback(fallback_404.into_service())
}

/// get /api/gshortlink/:id
/// post /api/shortlink
/// {"url":"hello.com"}
/// delete /api/shortlink
/// {"id":1}
pub fn short_links() -> Router {
    Router::new()
        .route(
            "/shortlink",post(shortlink_controller::create_shortlink),
        )
        .route(
            "/shortlink", routing::delete(shortlink_controller::delete_shortlink),
        )
        .route(
            "/shortlink/:id", get(shortlink_controller::get_shortlink))
        .route(
            "/shortlink",routing::put(shortlink_controller::update_shortlink),
        )
        .route(
            "/not_found", get(shortlink_controller::not_found))
}


/// jwt router
/// 
/// post /api/user/login
/// {"username":"admin","password":"admin"}
/// 
/// post /api/user/protected
/// {"id":1}
///  
pub fn user_links() -> Router {
    Router::new()
        .route("/user/login", post(user_controller::login))
        //.route("/user/register", get(user_controller::register))
        //.route("/user/logout", get(user_controller::logout))
        .route("/user/protected", post(user_controller::protected))
}

async fn fallback_404(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
