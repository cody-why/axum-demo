/*** 
 * @Author: plucky
 * @Date: 2022-07-02 21:11:56
 * @LastEditTime: 2022-07-07 01:44:24
 * @Description: 
 */

/// 获取当前时间戳
pub fn get_time_epoch()->u64{
    use std::time::SystemTime;
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
    .unwrap().as_secs()
}