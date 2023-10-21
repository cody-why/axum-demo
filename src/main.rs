/*
 * @Author: plucky
 * @Date: 2023-10-15 12:19:35
 * @LastEditTime: 2023-10-20 21:14:43
 */


use std::net::SocketAddr;
use tracing::info;

mod error;
mod api;
mod config;
mod utils;

pub use error::*;

use crate::api::routes;


#[tokio::main]
async fn main() {
    let config = config::load_config();

    config::init_log(&config.log);
    info!("{:?}", config);
    // config::db::init_sql_pool(&config.mysql).await.unwrap();
    // config::db::init_redis_pool(&config.redis).await.unwrap();
    
    let app = routes::app();

    let addr = SocketAddr::from(([127,0,0,1], config.server.port));
    info!("Server listening: {:?}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await.unwrap();
    
   
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    println!("shutting down...");
}


