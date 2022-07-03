/*** 
 * @Author: plucky
 * @Date: 2022-06-27 20:05:55
 * @LastEditTime: 2022-07-03 22:39:36
 * @Description: 
 */

#![allow(dead_code)]

use std::str::FromStr;

use serde::Deserialize;
use time::UtcOffset;
use tracing::Level;
use tracing_subscriber::fmt::time::OffsetTime;

pub mod routes;
pub mod database;


pub fn load_config() -> Config {
    serde_any::from_file::<Config,_>("app.yaml").unwrap()

}

pub fn init_tracing(config: &LogConfig) {
    //tracing_subscriber::fmt::init();

    // 设置输出时间为utc+8:00
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),

    );
    // 追踪日志
    let level = Level::from_str(&config.level).unwrap();
    tracing_subscriber::fmt().with_max_level(level)
    
    .with_target(false)
    .with_timer(local_time)
    //.with_timer(tracing_subscriber::fmt::time::uptime())// 计时器
    .with_line_number(true)
    .with_file(true)
    .init();
}

#[derive(Debug, Deserialize)]
pub struct Config {
   pub server: ServerConfig,
   pub mysql: MysqlConfig,
   pub redis: RedisConfig,
   pub log: LogConfig,
    
}





#[derive(Debug, Deserialize)]
pub struct ServerConfig{
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct MysqlConfig{
    url: String,
    max_connections: u32,
    min_connections: u32,
}
#[derive(Debug, Deserialize)]
pub struct RedisConfig{
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig{
    level: String,
}
