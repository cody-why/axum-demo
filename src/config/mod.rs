/*** 
 * @Author: plucky
 * @Date: 2022-06-27 20:05:55
 * @LastEditTime: 2022-07-08 21:42:59
 * @Description: 
 */

#![allow(dead_code)]

use std::{fmt::Debug};

use serde::Deserialize;
use time::{UtcOffset};

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter};
use tracing_subscriber::{ fmt::{time::OffsetTime, self}};
use tracing_subscriber::{
     layer::SubscriberExt, util::SubscriberInitExt, 
};
//filter::EnvFilter, 
pub mod routes;
pub mod database;

pub use database;

const CONFIGFILE: &str = "app.yaml";

pub fn load_config() -> Config {
    serde_any::from_file::<Config,_>(CONFIGFILE).unwrap()

}

pub fn init_log(config: &LogConfig)-> Option<WorkerGuard> {
    //println!("{}",cfg!(unsound_local_offset));
    // 设置输出时间为utc+8:00
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),

    );
    
    // 日志级别
    let env_filter = EnvFilter::new(&config.level);

    // 输出到控制台中
    let stdout_layer = fmt::layer().with_timer(local_time.clone())
    .pretty().with_writer(std::io::stderr);
    
    
    if config.tofile{
        // 输出到文件中
        let file_appender = tracing_appender::rolling::hourly("logs", "app.log");
        // non_blocking 在非阻塞的线程中输出日志。需要返回WorkerGuard，保证不会被销毁。
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        
        let file_layer = fmt::layer().with_timer(local_time)
        .with_line_number(true)
        .with_ansi(false)
        .with_writer(non_blocking);
        // 注册
        tracing_subscriber::Registry::default()
        .with(stdout_layer)
        .with(file_layer)
        .with(env_filter)
        .init();
        
        return Some(_guard);
    } 
    
     // 注册
     tracing_subscriber::Registry::default()
     .with(stdout_layer)
     .with(env_filter)
     .init();
    
    None

    // let sub = tracing_subscriber::fmt()
    // .with_max_level(Level::from_str(&config.level).unwrap())
    // .with_writer(non_blocking)
    // .with_target(false)
    // .with_timer(local_time)
    // .with_line_number(true)
    // .with_file(true)
    //.init();
    
}

#[derive(Debug, Deserialize)]
pub struct Config {
   pub server: ServerConfig,
   pub mysql: MysqlConfig,
   pub redis: RedisConfig,
   pub log: LogConfig,
    
}

// write with

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
    tofile: bool,
}
