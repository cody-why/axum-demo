/*
 * @Author: plucky
 * @Date: 2023-10-25 08:03:23
 * @LastEditTime: 2023-12-12 16:57:44
 */

#![allow(unused)]
use axum::{
	middleware::Next,
	response::{IntoResponse,Response}, extract::Request,
};
use tower_sessions::Session;

use crate::Error;

pub async fn check_auth(session: Session, req: Request, next: Next) -> Result<Response, Error>{
	// let mut can_access_bool  = can_access(token, uri.clone()).await;
	let can_access = false;
	

	if !can_access {
		return Err(Error::Unauthorized);
	}
	let response = next.run(req).await;
	Ok(response)
}
