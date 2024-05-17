/*
 * @Author: plucky
 * @Date: 2023-10-18 21:22:24
 * @LastEditTime: 2023-12-12 16:29:15
 */

use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, ToSchema)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

// #[allow(dead_code)]
// #[derive(Debug, validator::Validate, Deserialize)]
// struct SignupData {
//     #[validate(email)]
//     mail: String,
//     #[validate(phone)]
//     phone: String,
//     #[validate(url)]
//     site: String,
//     #[serde(rename = "firstName")]
//     first_name: String,
//     #[validate(range(min = 18, max = 20))]
//     age: u32,
// }

// match signup_data.validate() {
//     Ok(_) => (),
//     Err(e) => return e;
//   };