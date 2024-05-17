/*
 * @Author: plucky
 * @Date: 2023-10-25 18:15:57
 * @LastEditTime: 2023-10-26 20:46:11
 */

#![allow(unused_imports,unused_assignments)]
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};
use sqlx::Error;

/// 
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CasbinRule {
	pub id: u64,
	pub ptype: String,
	pub v0: String,
	pub v1: String,
	pub v2: String,
	pub v3: String,
	pub v4: String,
	pub v5: String,
}

impl<'c> FromRow<'c, MySqlRow> for CasbinRule {
	fn from_row(row: &MySqlRow) -> Result<Self, Error> {
		Ok(Role {
			id: row.get(0),
			ptype:row.get(1),
			v0: row.get(2),
			v1: row.get(3),
			v2: row.get(4),
			v3: row.get(5),
			v4: row.get(6),
			v5: row.get(7),
		})
	}
}

impl PartialEq for CasbinRule {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
