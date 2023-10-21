/*
 * @Author: plucky
 * @Date: 2023-10-17 20:12:30
 * @LastEditTime: 2023-10-18 23:18:31
 */

use axum::{
    extract::FromRequest,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use crate::{utils::get_time_epoch, Error};

use super::AUTH_TOKEN;


/// 环境变量密钥，
pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    // let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let secret = include_str!(".env");
    Keys::new(secret.as_bytes())
});

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[allow(unused)]
// 用中间件从cookie获取token
pub async fn auth<B>(cookies: Cookies, req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = cookies.get(AUTH_TOKEN);
    match auth_header {
        Some(auth_header) if token_is_valid(auth_header.value()) => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

// 用header获取token
// pub async fn auth<B>(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
//     if token_is_valid(auth.token()) {
//         Ok(next.run(req).await)
//     }
//     else {
//         Err(StatusCode::UNAUTHORIZED)
//     }
// }


fn token_is_valid(token: &str) -> bool {
    // debug!("auth-token: {:?}", token);
    // let bearer  = Authorization::bearer(token).unwrap();
    let claims = Claims::decode(token);
    if let Ok(claims) = claims {
        if claims.exp > get_time_epoch() {
            return true;
        }
    }

    false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub id: u64,
    pub username: String,
    pub exp: u64,
}

impl Claims {
    pub fn new(id: u64, username: String, exp: u64) -> Self {
        let exp = get_time_epoch() * exp;
        Self { id, username, exp }
    }
    pub fn decode(token: &str) -> Result<Self, Error> {
        let token_data = decode::<Claims>(token, &KEYS.decoding, &Validation::default())
            .map_err(|_| Error::InvalidToken)?
            .claims;
        Ok(token_data)
    }

    pub fn encode(&self) -> Result<String, Error> {
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), self, &KEYS.encoding).map_err(|_e| {
            Error::EncodeError
        })?;
        Ok(token)
    }
}

// impl FromRequest for Claims, 可以在路由直接注入Claims
#[axum::async_trait]
impl<S, B> FromRequest<S, B> for Claims
where
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        // let header = req.headers().get(header::AUTHORIZATION).ok_or(Error::InvalidToken)?;
        // let auth = header.to_str().unwrap_or("").trim_start_matches("Bearer ");
       
        // type Author = TypedHeader::<headers::Authorization<Bearer>>;
        // let bearer = Author::from_request(req, state).await.map_err(|_| Error::InvalidToken)?;

        // 从cookie中获取token
        let cookies = req.extensions().get::<Cookies>().ok_or(Error::InvalidToken)?;
        let auth = cookies.get(AUTH_TOKEN).ok_or(Error::InvalidToken)?;
        let auth = auth.value();

        let claims = Claims::decode(auth)?;
        if claims.exp < get_time_epoch() {
            return Err(Error::InvalidToken);
        }

        Ok(claims)
    }
}
