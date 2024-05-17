/*
 * @Author: plucky
 * @Date: 2023-10-25 18:29:56
 * @LastEditTime: 2024-03-18 09:46:03
 */

#![allow(unused_imports,unused_assignments)]
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};
use sqlx::Error;

/// 角色菜单关联表
#[derive(co_orm::Crud)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoleMenu {
	pub id: u64,
	pub role_id: u64,
	pub menu_id: u64,
}



impl<'c> FromRow<'c, MySqlRow> for RoleMenu {
	fn from_row(row: &MySqlRow) -> Result<Self, Error> {
		Ok(Permission {
			id: row.get(0),
			user_id: row.get(1),
			role_id: row.get(2),
		})
	}
}

impl PartialEq for RoleMenu {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
