/*
 * @Author: plucky
 * @Date: 2023-10-16 20:48:18
 * @LastEditTime: 2023-12-12 17:48:14
 */

use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde::Serialize;
use utoipa::ToSchema;


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginError(String),
    Unauthorized,
    InvalidToken,
    // EncodeError,
    DataBaseError,
    // Internal(code: u16, msg: String),
    INTERNAL(u16, String),
    Other(String),
    Bad400(String),
    
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // info!("IntoResponse error: {:?}", self);
        match self {
            Error::LoginError(_msg) => RespVO::error(StatusCode::UNAUTHORIZED, "LoginError").into_response(),
            Error::Unauthorized => RespVO::error(StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            Error::InvalidToken => RespVO::error(StatusCode::UNAUTHORIZED, "InvalidToken").into_response(),
            // Error::EncodeError => RespVO::error(StatusCode::INTERNAL_SERVER_ERROR, "EncodeError").into_response(),
            Error::DataBaseError => RespVO::error(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
            Error::INTERNAL(code, msg) => RespVO::error(code, msg).into_response(),
            Error::Other(msg) => RespVO::error(StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
            Error::Bad400(msg) => RespVO::error_400(msg).into_response(),
        }
    }
    
}

/// http接口返回 code，msg，data 等json数据结构
#[derive(Debug, Clone, Serialize,ToSchema)]
pub struct RespVO<T> {
    pub code: u16,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self::new(StatusCode::OK.as_u16(), "success", Some(data))
    }

    pub fn new(code: impl Into<u16>, msg: impl Into<String>, data: Option<T>) -> Self {
        Self {
            code: code.into(),
            msg: Some(msg.into()),
            data,
        }
    }

    // pub fn json(self) -> Json<Self> {
    //     Json(self)
    // }

}

impl RespVO<String>{
    pub fn error_400(msg: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, msg, None)
       
    }
    
    pub fn error(code: impl Into<u16>, msg: impl Into<String>) -> Self {
        Self::new(code, msg, None)
        
    }

    pub fn data(code: impl Into<u16>, msg: impl Into<String>, data: impl Into<String>) -> Self {
        Self::new(code, msg, Some(data.into()))
        
    }

}

impl<T> IntoResponse for RespVO<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

