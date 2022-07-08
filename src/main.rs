/*** 
 * @Author: plucky
 * @Date: 2022-06-27 16:34:07
 * @LastEditTime: 2022-07-08 21:09:53
 * @Descrip&&tion: 
 */

use std::error::Error;
use std::net::SocketAddr;



mod config;
mod app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config::load_config();

    let _wg = config::init_log(&config.log);
    
    tracing::debug!("{:#?}",config);

    let state = config::init_state(&config).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Server bind at: {:?}", addr);

    axum::Server::bind(&addr)
        .serve(config::routes::app(state).into_make_service())
        .await?;
    
    Ok(())
}