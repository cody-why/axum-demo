/*
 * @Author: plucky
 * @Date: 2023-10-18 09:41:49
 * @LastEditTime: 2023-10-20 17:28:10
 */



mod db;

/// 获取当前时间戳
pub fn get_time_epoch()->u64{
    use std::time::SystemTime;
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
    .unwrap().as_secs()
}

