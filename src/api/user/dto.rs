/*
 * @Author: plucky
 * @Date: 2023-10-18 21:22:24
 * @LastEditTime: 2023-10-19 09:35:12
 */

#[derive(serde::Deserialize, Debug)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}
