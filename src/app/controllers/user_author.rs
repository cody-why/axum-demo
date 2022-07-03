/*** 
 * @Author: plucky
 * @Date: 2022-07-02 22:12:45
 * @LastEditTime: 2022-07-02 23:16:16
 * @Description: 
 */

use std::fmt::Display;

use axum::{extract::{FromRequest, RequestParts}, TypedHeader, response::{IntoResponse, Response}, http::StatusCode, Json};
use headers::authorization::Bearer;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use jsonwebtoken as jwt;


// 懒加载一个全局的jwt加解密密钥
pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = "hello user";
    Keys::new(secret.as_bytes())
});

// 定义一个jwt配置结构体
// #[allow(dead_code)]
pub struct Keys {
    pub encoding: jwt::EncodingKey,
    pub decoding: jwt::DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: jwt::EncodingKey::from_secret(secret),
            decoding: jwt::DecodingKey::from_secret(secret),
        }
    }
}


/// 编码 Claims to token
///
/// 返回: token
/// 抛出: AuthError
pub fn encode_claims(claims: &Claims) -> Result<String, AuthError> {
    let token = jwt::encode(&jwt::Header::default(), claims, &KEYS.encoding).map_err(|e| {
        tracing::error!("encode_claims error: {:?}", e);
        AuthError::EncodeError
    })?;
    Ok(token)
}

/// 权限结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: u64,
    pub name: String,
    pub exp: u64,
}

// 实现Claims的new方法
impl Claims {
    
    pub fn new(id: u64, name: String, exp: u64) -> Self {
        Self {
            id,
            name,
            exp,
        }
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{id: {}, Name: {}}}", self.id, self.name)
    }
}

// 实现Claims的FromRequest方法,从请求中获取token,解码为Claims
#[axum::async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // 这几步从header获取Authorization的值，然后解析出token
        // let auth = req.headers().get("Authorization").ok_or(AuthError::NoToken)?;
        // println!("{:?}", auth);
        // let auth = auth.to_str().unwrap_or("").trim_start_matches("Bearer ");
        // let bearer  = Authorization::bearer(auth).unwrap();
        // println!("{:?}", bearer.token());
        
        // 更简洁的方式获取token
        type Author=TypedHeader::<headers::Authorization<Bearer>>;
        let bearer = Author::from_request(req).await
        .map_err(|_| AuthError::InvalidToken)?;
        // decode the token
        let token = jwt::decode::<Claims>(&bearer.token(), &KEYS.decoding, &jwt::Validation::default()).map_err(|_e| AuthError::InvalidToken)?;
       
        Ok(token.claims)
    }
}

// 认证错误类型
#[derive(Debug)]
pub enum AuthError {
    //NoToken,
    InvalidToken,
    EncodeError,
}

// 实现AuthError的IntoResponse方法,返回错误信息
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            //AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            //AuthError::NoToken => (StatusCode::BAD_REQUEST, "Missing token"),
            AuthError::EncodeError => (StatusCode::BAD_REQUEST, "Encode token error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrResp {
    pub error: String
}
