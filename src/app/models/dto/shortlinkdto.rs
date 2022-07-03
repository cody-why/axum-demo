/*** 
 * @Author: plucky
 * @Date: 2022-06-27 20:13:59
 * @LastEditTime: 2022-07-02 00:21:26
 * @Description: 
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateShortLinkReq {
    pub url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResp {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteShortLinkReq {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteShortLinkResp {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortLinkInfo {
    pub id: u32,
    pub url: String
}
