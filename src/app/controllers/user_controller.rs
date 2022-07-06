/*** 
 * @Author: plucky
 * @Date: 2022-07-01 19:34:54
 * @LastEditTime: 2022-07-04 12:15:57
 * @Description: 
 */

use std::sync::Arc;
use axum::{Json, Extension, response::{IntoResponse}, http::{StatusCode}};

use crate::app::{models::dto::userdto, utils::*, state::State};
use super::user_author::*;


/// [post]登录
/// body: {username:"",password:""}
pub async fn login(
    Json(req): Json<userdto::LoginReq>,
    Extension(_state): Extension<Arc<State>>,
) -> impl IntoResponse {
    tracing::info!("user_login {:?}", req);

    // 创建一个24小时的token
    let claims  =  Claims::new(1, req.username,get_time_epoch()*3600*24);
    let token = encode_claims(&claims).unwrap();
    
    (StatusCode::OK, Json(userdto::LoginResp {ok:true, token:token }))
    
}

/// [get]查询用户信息
/// header: {Authorization:"Bearer xxxxx"}
pub async fn protected(claims: Claims) -> Result<String, AuthError> {
    tracing::info!("user_protected {:?}", claims);

    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{}",
        claims
    ))
}

