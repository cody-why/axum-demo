/*
 * @Author: plucky
 * @Date: 2023-10-25 08:08:05
 * @LastEditTime: 2023-10-30 09:08:36
 */
#![allow(unused)]
use bcrypt::{DEFAULT_COST, hash, verify};
use crate::{Result, Error};


pub async fn verify_password(password: String, hashed: String) -> Result<bool> {
	verify(password, &hashed).map_err(|e| Error::LoginError("password error".into()))
}

pub async fn encrypt_password(password: String) -> Result<String> {
	hash(password, DEFAULT_COST).map_err(|e|Error::Other(e.to_string()))
}
