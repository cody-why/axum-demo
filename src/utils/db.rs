/*
 * @Author: plucky
 * @Date: 2023-10-20 17:27:54
 * @LastEditTime: 2023-10-27 21:28:43
 */

/// sqlx::query_as的调用
/// ``` no_run
/// query_as!(User, "select * from users where name = ?", name).fetch_one(&pool).await?;
/// ```
#[macro_export]
macro_rules! query_as (
    ($out_struct:path, $query:expr) => ( {
        sqlx::query_as::<_, $out_struct>($query)
    });
    ($out_struct:path, $query:expr, $($args:expr),*) => ( {
        sqlx::query_as::<_, $out_struct>($query)
        $(.bind($args))*
    })
);

/// sqlx::query的调用
/// ``` no_run
/// query!("insert into users (name, password) values (?,?)", name, password).execute(&pool).await?;
/// ```
#[macro_export]
macro_rules! query (
    ($query:expr) => ( {
        sqlx::query($query)
    });
    ($query:expr, $($args:expr),*) => ( {
        sqlx::query($query)
        $(.bind($args))*
    })
);
