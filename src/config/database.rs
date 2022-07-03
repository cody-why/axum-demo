use redis::aio::Connection;
/*** 
 * @Author: plucky
 * @Date: 2022-06-27 20:03:53
 * @LastEditTime: 2022-07-01 11:29:02
 * @Description: 
 */
use sqlx::mysql::{MySqlPoolOptions};
use sqlx::{MySql, Pool};

pub async fn do_connect() -> Pool<MySql> {
    // let mut opt =  MySqlConnectOptions::new().
    // host("192.168.1.166")
    // .port(13306)
    // .username("root")
    // .password("789789")
    // .database("hello");
    // opt.log_statements(tracing::log::LevelFilter::Off);
   
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        //.connect_with(opt).await;
        .connect("mysql://root:newpassword@192.168.1.199:3306/hello").await;
       //mysql://user:pwd@192.168.1.110/db
 
    pool.unwrap()
}

pub async fn do_redis_connect() -> Connection {
    //redis://user:pwd@192.168.1.110/db
    let client = redis::Client::open("redis://:789789@192.168.1.199/").unwrap();
    println!("{:?}",client.get_connection_info());
    client.get_async_connection().await.unwrap()
}
