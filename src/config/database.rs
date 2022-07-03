/*** 
 * @Author: plucky
 * @Date: 2022-06-27 20:03:53
 * @LastEditTime: 2022-07-03 22:21:00
 * @Description: 
 */



use sqlx::mysql::{MySqlPoolOptions};
use sqlx::{MySql, Pool};
use redis::aio::{ConnectionManager};
use crate::app::state;

use super::*;



pub async fn init_state(config: &Config)->state::State{
    let mysql_pool = init_mysql_pool(&config.mysql).await;
    let redis_pool = init_redis_pool(&config.redis).await;
    state::State::new(mysql_pool, redis_pool)
   
}

pub async fn init_mysql_pool(config: &MysqlConfig) -> Pool<MySql> {
    // let mut opt =  MySqlConnectOptions::new().
    // opt.log_statements(tracing::log::LevelFilter::Off);
   
    // mysql://user:pwd@host:port/db
    let pool = MySqlPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        //.connect_with(opt).await;
        .connect(&config.url).await;
    tracing::debug!("mysql pool: {:?}", pool);
    pool.unwrap()
}

pub async fn init_redis_pool(config: &RedisConfig) -> ConnectionManager {
    // redis://user:pwd@host:port/db
    let client = redis::Client::open(config.url.as_str()).unwrap();

    tracing::debug!("{:?}",client.get_connection_info());
    // client.get_async_connection().await.unwrap()
    
    client.get_tokio_connection_manager().await.unwrap()
    
}
