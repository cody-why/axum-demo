pub mod db;
/*
 * @Author: plucky
 * @Date: 2023-10-16 21:09:50
 * @LastEditTime: 2023-10-18 22:37:07
 */

use serde::Deserialize;

mod log;
pub use log::*;

const CONFIGFILE: &str = "app.yaml";

pub fn load_config() -> Config {
    let path = std::env::current_exe().unwrap().parent().unwrap().join("");
    println!("current_exe: {:?}", path);

    #[cfg(not(debug_assertions))]
    {
        std::env::set_current_dir(path).unwrap();
    }
    println!("current_dir: {:?}", std::env::current_dir().unwrap());

    // serde_any::from_file::<Config,_>(CONFIGFILE).unwrap_or_default()
    let str = std::fs::read_to_string(CONFIGFILE).unwrap_or_default();
    serde_yaml::from_str(&str).unwrap_or_default()
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub server: ServerConfig,
    pub log: LogConfig,
    pub mysql: SqlConfig,
    pub redis: RedisConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self { port: 8080 }
    }
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub tofile: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            tofile: false,
        }
    }
}

#[derive(Debug,Default, Deserialize)]
pub struct SqlConfig{
    url: String,
    max_connections: u32,
    min_connections: u32,
}

#[derive(Debug,Default, Deserialize)]
pub struct RedisConfig{
    url: String,
}
