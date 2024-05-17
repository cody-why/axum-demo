/*
 * @Author: plucky
 * @Date: 2023-10-23 19:47:19
 * @LastEditTime: 2024-03-18 09:44:06
 */

use crate::{Result, Error, routes::auth::jwt::Claims, RespVO};
use axum::{Router, response::IntoResponse, routing::get};
use tracing::debug;

use super::dao::menu::SysMenu;

/// routes
pub fn routes() -> Router {
    Router::new()
        .route("/menu/query/tree", get(query_tree))
        
}

// query all menu
async fn query_tree(_claims: Claims) -> Result<impl IntoResponse> {
    let menu  = SysMenu::list().await.map_err(|e|{
        debug!("SysMenu::list: {:?}", e);
        Error::DataBaseError
    })?;
    Ok(RespVO::success(menu))
}
