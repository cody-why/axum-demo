/*
 * @Author: plucky
 * @Date: 2023-10-20 17:27:54
 * @LastEditTime: 2023-10-20 18:36:10
 */

/// 简化sqlx::query_as的调用,query_as!(User, "select * from users where name = ?", name)
#[macro_export]
macro_rules! query_as (
    ($out_struct:path, $query:expr) => ( {
        sqlx::query_as::<_, $out_struct>($query)
    });
    ($out_struct:path, $query:expr, $($args:tt)*) => ( {
        sqlx::query_as::<_, $out_struct>($query)
        $(.bind($args))*
    })
);

