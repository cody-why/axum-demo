use std::sync::Arc;

use axum::body::Body;
/***
 * @Author: plucky
 * @Date: 2022-06-27 20:13:16
 * @LastEditTime: 2022-07-01 00:25:33
 * @Description:
 */
use axum::{extract, Json};

use redis::{AsyncCommands, RedisResult};

use crate::app::models::dto::shortlinkdto;
use crate::app::models::shortlink;
use crate::app::state::State;
use axum::extract::Extension;
use axum::http::header::LOCATION;
use axum::http::{HeaderMap, Request, StatusCode};
use axum::response::IntoResponse;


/// [post]创建一个链接,并且保存到数据库
/// body: {url:""}
///
pub async fn create_shortlink(
    Json(req): Json<shortlinkdto::CreateShortLinkReq>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    tracing::info!("create_shortlink {:?}", req);

    let url = if !req.url.starts_with("http"){
        format!("http://{}", req.url)
    }else{
        req.url
    };
    
    match shortlink::create_shortlink(&state.pool, &url).await {
        Ok(_) => (StatusCode::OK, Json(shortlinkdto::CreateUserResp { ok: true })),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(shortlinkdto::CreateUserResp { ok: false }),
        ),
    }
}

/// [post]删除一个链接
/// body: {id:1}
///
pub async fn delete_shortlink(
    Json(req): Json<shortlinkdto::DeleteShortLinkReq>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    tracing::info!("delete_shortlink {:?}", req);
    
    let id = req.id;
    match shortlink::delete_shortlink(&state.pool, id).await {
        Ok(_) => {
            // let mut redis_conn = state.redis_conn.lock().await;
            let mut redis_conn = state.redis_pool.clone();
            let _ = redis_conn.del::<_,()>(&id.to_string()).await;

            (StatusCode::OK, Json(shortlinkdto::DeleteShortLinkResp { ok: true }))
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(shortlinkdto::DeleteShortLinkResp { ok: false }),
        ),
    }
}


/// [get]查询链接,/:id
///
pub async fn get_shortlink(
    extract::Path(id): extract::Path<i32>,
    req: Request<Body>,
) -> impl IntoResponse {
    tracing::info!("get_shortlink/{:?}", id);
    let state =  req.extensions().get::<Arc<State>>().unwrap();
    state.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    tracing::info!("{:?}", state.counter);

    let mut url = "/api/not_found".to_string();
    
    let redis_key = format!("url_{}",id);
    let mut redis_conn = state.redis_pool.clone();

    let res: RedisResult<String>;
    {
        //let mut redis_conn = state.redis_conn.lock().await;å
        res = redis_conn.get(&redis_key).await;
    }
    // con.set("key", 1); 报错type annotations needed cannot satisfy `_: FromRedisValue`
    // 意思是不知道泛型的FromRedisValue是什么类型,因为他的返回值RV没有指定值类型
    // 解决方法1: 指定返回值类型,let r:redis::RedisResult<()> 方法2:使用泛型con.set::<_,_,()>();

    match res {
        Ok(v) => {
            url = v;
        }
        Err(err) => {
            println!("Redis err = {:?}", err);
            
            match shortlink::get_shortlink(&state.pool, id).await {
                Ok(record) => {
                    //url = Box::leak(record.url.into_boxed_str());
                    url = if !record.url.starts_with("http"){
                        format!("http://{}", record.url)
                    }else{
                        record.url
                    };
                    //let mut redis_conn = state.redis_conn.lock().await;
                    let _:RedisResult<()> = redis_conn.set(&redis_key, &url).await;
                }
                Err(err) => {
                    println!("Mysql err = {:?}", err);
                }
            }
        }
    }

    

    //返回头部,跳转到短链接
    let mut headers = HeaderMap::new();
    headers.insert(LOCATION, url.parse().unwrap());

    // FOUND=302,
    (StatusCode::FOUND, headers)
}

/// 更新链接
pub async fn update_shortlink(
    Json(req): Json<shortlinkdto::ShortLinkInfo>,
    parts: axum::http::request::Parts,
) -> impl IntoResponse {
    tracing::info!("update_shortlink {:?}", req);

    let state =  parts.extensions.get::<Arc<State>>().unwrap();
    
    // let mut redis_conn = state.redis_conn.lock().await;
    let redis_key = format!("url_{}",req.id);
    let mut redis_conn = state.redis_pool.clone();
    _=redis_conn.del::<_,()>(redis_key).await;

    match shortlink::update_shortlink(&&state.pool, req.id, &req.url).await {
        Ok(_) => (StatusCode::OK, Json(shortlinkdto::CreateUserResp { ok: true })),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(shortlinkdto::CreateUserResp { ok: false }),
        ),
    }

    


}

/// 数据不存在
pub async fn not_found() -> impl IntoResponse {
    (StatusCode::OK, "404 Not Found")
}
