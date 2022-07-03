/*** 
 * @Author: plucky
 * @Date: 2022-07-01 22:06:21
 * @LastEditTime: 2022-07-02 22:14:00
 * @Description: 
 */


use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct LoginReq {
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResp {
    pub ok: bool,
    pub token: String
}