/*
 * @Author: plucky
 * @Date: 2023-10-18 09:41:49
 * @LastEditTime: 2023-10-25 08:44:42
 */



mod db;
mod router_util;
mod encrypt_util;

/// 获取当前时间戳
pub fn get_timestamp()->i64{
    chrono::Utc::now().timestamp()
    // use std::time::SystemTime;
    // SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

