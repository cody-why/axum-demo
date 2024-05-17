
/*
 * @Author: plucky
 * @Date: 2023-10-16 21:09:50
 * @LastEditTime: 2024-02-29 17:56:38
 */

use std::env::args;

use serde::Deserialize;

mod log;
pub use log::*;
pub mod db;
pub use db::*;


pub fn load_config() -> Config {
    let config_file = args().nth(1).unwrap_or_else(|| "app.yaml".to_string());
    

    #[cfg(not(debug_assertions))]
    {
        let path = std::env::current_exe().unwrap().parent().unwrap().join("");
        // println!("current_exe: {:?}", path);
        std::env::set_current_dir(path).unwrap();
    }
    println!("current_dir: {:?}", std::env::current_dir().unwrap());


    let file = std::fs::File::open(config_file).expect("failed to open file");
    serde_yaml::from_reader(file).expect("failed to parse file")
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub server: ServerConfig,
    pub log: LogConfig,
    pub mysql: DBConfig,
    pub redis: RedisConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub addr: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self { addr: "127.0.0.1:8080".to_string() }
    }
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub file_path: String,
    pub file_name: String,
    pub log_to_file: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            file_path: "logs".to_string(),
            file_name: "app.log".to_string(),
            log_to_file: false
        }
    }
}

#[derive(Debug,Default, Deserialize)]
pub struct DBConfig{
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug,Default, Deserialize)]
pub struct RedisConfig{
    url: String,
}
