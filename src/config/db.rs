/*
 * @Author: plucky
 * @Date: 2023-10-18 22:33:53
 * @LastEditTime: 2023-10-19 11:33:56
 */
#![allow(unused)]
use once_cell::sync::OnceCell;

use redis::{RedisResult, aio::ConnectionManager};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool, Error};
use tracing::debug;

use super::{SqlConfig, RedisConfig};

static MYSQL_POOL: OnceCell<MySqlPool> = OnceCell::new();
static REDIS_POOL: OnceCell<ConnectionManager> = OnceCell::new();

//建立mysql连接
pub async fn init_sql_pool(config: &SqlConfig) -> Result<(), Error>  {
    // let mut opt =  MySqlConnectOptions::new().
    // opt.log_statements(tracing::log::LevelFilter::Off);
   
    // mysql://user:pwd@host:port/db
    let pool = MySqlPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        //.connect_with(opt).await;
        .connect(&config.url).await?;
    debug!("sql pool: {:?}", pool);
    assert!(MYSQL_POOL.set(pool).is_ok());
    Ok(())
}

//获取数据库
pub fn get_pool() -> Result<&'static MySqlPool, Error> {
    MYSQL_POOL.get().ok_or(Error::PoolTimedOut)
}

pub async fn init_redis_pool(config: &RedisConfig) -> RedisResult<()> {
    // redis://user:pwd@host:port/db
    let client = redis::Client::open(config.url.as_str()).unwrap();

    debug!("{:?}",client.get_connection_info());
    // client.get_async_connection().await.unwrap()
    
    let pool = client.get_tokio_connection_manager().await?;
    
    assert!(REDIS_POOL.set(pool).is_ok());
    Ok(())
}

pub fn get_redis_pool() -> Option<&'static ConnectionManager> {
    REDIS_POOL.get()
}