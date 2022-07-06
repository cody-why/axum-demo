/*** 
 * @Author: plucky
 * @Date: 2022-06-27 20:49:16
 * @LastEditTime: 2022-07-07 01:44:44
 * @Description: 全局共享状态
 */

use redis::aio::{ ConnectionManager};
use sqlx::{Pool, MySql};
//use tokio::sync::Mutex;




// Some shared state used throughout our application
#[allow(dead_code)]
pub struct State {
    pub counter: std::sync::atomic::AtomicU64,
    pub pool: Pool<MySql>,
    //pub redis_pool: Mutex<Connection>,
    pub redis_pool: ConnectionManager,
}

impl State {
    pub fn new(pool: Pool<MySql>, redis_pool: ConnectionManager) -> Self {
        //let redis_conn = Mutex::new(redis_conn);
        State {
            counter: std::sync::atomic::AtomicU64::new(0),
            pool,
            redis_pool,
        }
    }
}
    

