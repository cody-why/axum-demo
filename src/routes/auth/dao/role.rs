/*
 * @Author: plucky
 * @Date: 2023-10-25 18:15:57
 * @LastEditTime: 2024-03-18 09:43:07
 */

#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};
use sqlx::Error;

/// 角色
#[derive(co_orm::Crud)]
#[co_orm(rename= "sys_role")]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Role {
	pub id: u64,
	pub code: String,
	pub name: String,
	pub is_delete: bool,
}


impl<'c> FromRow<'c, MySqlRow> for Role {
	fn from_row(row: &MySqlRow) -> Result<Self, Error> {
		Ok(Role {
			id: row.get(0),
			code:row.get(1),
			name: row.get(2),
			is_delete: row.get(3),
		})
	}
}

impl PartialEq for Role {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
