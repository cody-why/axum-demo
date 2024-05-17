/*
 * @Author: plucky
 * @Date: 2023-10-25 18:29:56
 * @LastEditTime: 2024-03-18 09:45:41
 */

#![allow(unused)]
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};
use sqlx::Error;

/// 用户角色
#[derive(co_orm::Crud)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRole {
	pub id: u64,
	pub user_id: u64,
	pub role_id: u64,
}



impl<'c> FromRow<'c, MySqlRow> for UserRole {
	fn from_row(row: &MySqlRow) -> Result<Self, Error> {
		Ok(Permission {
			id: row.get(0),
			user_id: row.get(1),
			role_id: row.get(2),
		})
	}
}

impl PartialEq for UserRole {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
