/*
 * @Author: plucky
 * @Date: 2023-10-23 19:35:39
 * @LastEditTime: 2023-12-12 17:22:13
 */
pub mod jwt;
pub mod dao;
mod router;
pub use router::*;

pub const AUTH_TOKEN: &str = "token";
