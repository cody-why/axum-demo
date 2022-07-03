use redis::aio::Connection;
use sqlx::{Pool, MySql};
use tokio::sync::Mutex;

/*** 
 * @Author: plucky
 * @Date: 2022-06-27 20:49:16
 * @LastEditTime: 2022-07-02 16:48:56
 * @Description: 全局共享状态
 */
// Some shared state used throughout our application
#[allow(dead_code)]
pub struct State {
    pub counter: std::sync::atomic::AtomicU64,
    pub pool: Pool<MySql>,
    pub redis_conn: Mutex<Connection>,
}

impl State {
    pub fn new(pool: Pool<MySql>, redis_conn: Connection) -> Self {
        let redis_conn = Mutex::new(redis_conn);
        State {
            counter: std::sync::atomic::AtomicU64::new(0),
            pool,
            redis_conn,
        }
    }
}
    

