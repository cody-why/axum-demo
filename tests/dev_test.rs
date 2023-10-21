/*
 * @Author: plucky
 * @Date: 2023-10-16 22:32:06
 * @LastEditTime: 2023-10-18 16:44:56
 */

#![allow(unused)]
use serde_json::json;


#[tokio::test]
async fn quick_dev()-> anyhow::Result<()>{
    let hc = httpc_test::new_client("http://localhost:8080")?;
    let content = json!({
        "username": "plucky",
        "password": "123456"
    });
    hc.do_post("/api/login", content).await?.print().await?;

    hc.do_get("/api/check").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn check_test(){
    let hc = httpc_test::new_client("http://localhost:8080").unwrap();
    let res = hc.do_get("/api/check").await.unwrap();

    let auth_token = res.res_cookie_value("auth-token"); 
	let content_type = res.header("content_type");
    
    res.print().await.unwrap();
}
    