/*
 * @Author: plucky
 * @Date: 2022-09-05 09:23:36
 * @LastEditTime: 2023-12-11 17:33:20
 * @Description: 
 */

use time::UtcOffset;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::{time::OffsetTime, self};
use tracing_subscriber::{
     layer::SubscriberExt, util::SubscriberInitExt, 
};
// use  tracing_subscriber::filter::LevelFilter;

use super::*;



pub fn init_log(config: &LogConfig) {
    // 设置输出时间为utc+8:00
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),

    );
    
    // 日志级别 my_crate=info
    let env_filter = EnvFilter::new(&config.level);
  

    // 输出到控制台中
    let stdout_layer = fmt::layer().with_timer(local_time.clone())
        .pretty().with_writer(std::io::stderr);
    
    
    if config.log_to_file{
        // 输出到文件中
        let file_appender = tracing_appender::rolling::daily(&config.file_path, &config.file_name);
        // non_blocking 在非阻塞的线程中输出日志。
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
        // 防止WorkerGuard被销毁
        Box::leak(Box::new(_guard));
        
        return;
    } 
    
     // 注册
     tracing_subscriber::Registry::default()
     .with(stdout_layer)
     .with(env_filter)
     .init();
    
    
}
