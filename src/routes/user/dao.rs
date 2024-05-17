/*
 * @Author: plucky
 * @Date: 2023-10-19 11:15:43
 * @LastEditTime: 2024-05-17 15:52:35
 */
#![allow(unused)]
use chrono::NaiveDateTime;
use sqlx::Error;
use crate::{query_as, config, query};
use crate::config::db::get_pool;

// user module
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn new(
        id: u64,
        name: String,
        password: String,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>, 
    ) -> Self {
        Self {
            id,
            name,
            password,
            created_at,
            updated_at,
        }
    }

    pub async fn list() -> Result<Vec<User>, Error> {
        let pool = get_pool()?;
        let users = query_as!(User,"select * from users").fetch_all(pool).await?;
        Ok(users)
    }
   

    pub async fn find_by_name(name: &str) -> Result<User, Error> {
        let pool = get_pool()?;
        let user =query_as!(User, "select * from users where name = ?", name)
            .fetch_one(pool)
            .await?;
        
        Ok(user)
    }

    pub async fn insert(&self) -> Result<u64, Error> {
        let pool = get_pool()?;
        let id = query!("insert into users (name, password) values (?,?)", &self.name, &self.password)
            .execute(pool)
            .await?;
        Ok(id.last_insert_id())
    }

    pub async fn delete(name: &str) -> Result<u64, Error> {
        let pool = get_pool()?;
        let id = query!("delete from users where name = ?", name).execute(pool).await?;
           
        Ok(id.last_insert_id())
    }
    
    pub async fn insert_all(users: Vec<User>) -> Result<u64, Error> {
        let pool = get_pool()?;
        let mut qb = sqlx::QueryBuilder::new("insert into users (name, password)");

        qb.push_values(users, |mut q, user| {
            q.push_bind(user.name).push_bind(user.password);
        });
        let id = qb.build().execute(pool).await?;

        Ok(id.rows_affected())
    }

    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list() {
        let config = config::load_config();
        config::db::init_db_pool(&config.mysql).await;

        let user = User::list().await;
        println!("{:?}", user);
    }

    #[tokio::test]
    async fn test_find_by_name() {
        let config = config::load_config();
        config::db::init_db_pool(&config.mysql).await;
        // User::delete("jack4").await;

        let user = User::find_by_name("jack1").await;
        println!("{:?}", user);
    }


    #[tokio::test]
    async fn test_insert() {
        let config = config::load_config();
        config::db::init_db_pool(&config.mysql).await;

        let time= chrono::Utc::now().naive_utc();
        let users = vec![
            User::new(1, "jack4".to_string(), "123456".to_string(), Some(time), None),
            // User::new(1, "jack2".to_string(), "123456".to_string(), Some(time), Some(time)),
        ];
        let id = User::insert_all(users).await;
        println!("{:?}", id);
    }
}