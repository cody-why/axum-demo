/*
 * @Author: plucky
 * @Date: 2023-10-16 20:48:18
 * @LastEditTime: 2023-10-18 09:35:28
 */

use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

//常量
pub const CODE_SUCCESS: StatusCode = StatusCode::OK;
pub const CODE_FAIL: StatusCode = StatusCode::BAD_REQUEST;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginError,
    Unauthorized,
    InvalidToken,
    EncodeError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // info!("IntoResponse error: {:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        // RespVO::from_error_info(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
    }
    
}

/// http接口返回模型结构，提供基础的 code，msg，data 等json数据结构
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RespVO<T> {
    pub code: Option<u16>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &T) -> Self {
        Self {
            code: Some(CODE_SUCCESS.as_u16()),
            msg: Some("success".to_string()),
            data: Some(arg.clone()),
        }
    }

    pub fn from_error(arg: &str) -> Self {
        Self {
            code: Some(CODE_FAIL.as_u16()),
            msg: Some(arg.to_string()),
            data: None,
        }
    }

    pub fn from_error_info(code: StatusCode, info: &str) -> Self {
        Self {
            code: Some(code.as_u16()),
            msg: Some(info.to_string()),
            data: None,
        }
    }
    pub fn to_json(self) -> Json<Self> {
        Json(self)
    }

}
