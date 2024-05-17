/*
 * @Author: plucky
 * @Date: 2023-10-17 20:12:30
 * @LastEditTime: 2024-03-01 21:07:50
 */

use std::sync::OnceLock;

use axum::{
    extract::{FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
// use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use crate::{utils::*, Error};

use super::AUTH_TOKEN;


// static KEYS: Lazy<Keys> = Lazy::new(|| {
//     // let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
//     let secret = include_str!(".env");
//     Keys::new(secret.as_bytes())
// });

// static KEYS: LazyLock<u32> = LazyLock::new(|| 0u32);

fn get_key()-> &'static Keys {
    static KEYS: OnceLock<Keys> = OnceLock::new();

    KEYS.get_or_init(|| {
        // let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let secret = include_str!(".env");
        Keys::new(secret.as_bytes())

    })
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub id: u64,
    pub username: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(id: u64, username: String, exp: i64) -> Self {
        let iat = get_timestamp();
        let exp = iat + exp;
        Self { id, username, exp, iat }
    }
    
    pub fn decode(token: &str) -> Result<Self, Error> {
        let token_data = decode::<Claims>(token, &get_key().decoding, &Validation::default())
            .map_err(|_| Error::InvalidToken)?
            .claims;
        Ok(token_data)
    }

    pub fn encode(&self) -> Result<String, Error> {
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), self, &get_key().encoding).map_err(|_e| {
            Error::InvalidToken
        })?;
        Ok(token)
    }
}

// impl FromRequest for Claims, 可以在路由直接注入Claims
#[axum::async_trait]
impl<S> FromRequestParts<S> for Claims
where
    // B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 从header中获取token
        let header = parts.headers.get(AUTH_TOKEN).ok_or(Error::InvalidToken)?;
        let auth = header.to_str().unwrap_or(""); //.trim_start_matches("Bearer ");
        
        // type Author = TypedHeader::<headers::Authorization<Bearer>>;
        // let bearer = Author::from_request(req, state).await.map_err(|_| Error::InvalidToken)?;

        // 从cookie中获取token
        // let cookies = req.extensions().get::<Cookies>().ok_or(Error::InvalidToken)?;
        // let auth = cookies.get(AUTH_TOKEN).ok_or(Error::InvalidToken)?;
        // let auth = auth.value();

        let claims = Claims::decode(auth)?;
        if claims.exp < get_timestamp() {
            return Err(Error::INTERNAL(401, "token expired".to_string()));
        }

        Ok(claims)
    }
}


#[allow(unused)]
// 用中间件从cookie获取token
pub async fn auth(cookies: Cookies, req: Request, next: Next) -> Result<Response, StatusCode> {
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
    let claims = Claims::decode(token);
    if let Ok(claims) = claims {
        if claims.exp > get_timestamp() {
            return true;
        }
    }

    false
}

