/*
 * @Author: plucky
 * @Date: 2023-10-15 12:19:35
 * @LastEditTime: 2024-02-29 17:35:17
 */


use std::net::SocketAddr;
use tokio::signal;
use tracing::info;

mod error;
mod routes;
mod config;
mod utils;
mod middlewares;


pub use error::*;

use crate::routes::root;


#[tokio::main]
async fn main() {
    let config = config::load_config();

    config::init_log(&config.log);
    info!("{:?}", config);
    config::init_db_pool(&config.mysql).await.unwrap();
    // config::init_redis_pool(&config.redis).await.unwrap();
    
    let app = root::app();
    // println!("{:#?}", app);

    let addr = config.server.addr.parse::<SocketAddr>().unwrap();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("Server listening: {:?}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
   
}




async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}