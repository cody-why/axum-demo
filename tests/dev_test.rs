/*
 * @Author: plucky
 * @Date: 2023-10-16 22:32:06
 * @LastEditTime: 2024-03-01 21:05:46
 */

#![allow(unused)]
use std::collections::HashMap;

use serde_json::{json, Value};


#[tokio::test]
async fn quick_dev()-> anyhow::Result<()>{
    let hc = httpc_test::new_client("http://localhost:8080/api")?;
    let content = json!({
        "username": "plucky",
        "password": "123456"
    });
    let res = hc.do_post("/login", content).await?;
    let token = res.header("token").unwrap();
    res.print().await?;

    
    Ok(())
}

#[tokio::test]
async fn check_test()-> anyhow::Result<()>{
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwidXNlcm5hbWUiOiJwbHVja3kiLCJleHAiOjE3MDI0Nzk1NDYsImlhdCI6MTcwMjM5MzE0Nn0.RVAhw0H2HZP2x_WzuDwA6K5lYk1jsiGGPyAwCWYJX1E";
    let builder = reqwest::Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("token", token.parse().unwrap());
            headers
        });
    let hc = httpc_test::new_client_with_reqwest("http://localhost:8080/api", builder).unwrap();
    hc.do_get("/check").await?.print().await?;
    

    Ok(())
}
    