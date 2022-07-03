/*** 
 * @Author: plucky
 * @Date: 2022-06-27 16:34:07
 * @LastEditTime: 2022-07-02 23:09:31
 * @Description: 
 */
use std::error::Error;
use std::net::SocketAddr;
use time::{UtcOffset};
use tracing::{Level, info};
use tracing_subscriber::fmt::time::{OffsetTime};

use crate::app::state::State;


mod config;
mod app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //tracing_subscriber::fmt::init();

    // 设置输出时间为utc+8:00,本地时间
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]")?

    );
    // 追踪日志
    tracing_subscriber::fmt().with_max_level(Level::DEBUG)
    .with_target(false)
    .with_timer(local_time)
    //.with_timer(tracing_subscriber::fmt::time::uptime())// 计时器
    .with_line_number(true)
    .with_file(true)
    .init();
    
    let pool = config::database::do_connect().await;
    let redis_conn = config::database::do_redis_connect().await;
    info!("database connected! {:?}", pool);
    let state = State::new(pool, redis_conn);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Server bind at: {:?}", addr);

    axum::Server::bind(&addr)
        .serve(config::routes::app(state).into_make_service())
        .await?;
    
    Ok(())
}