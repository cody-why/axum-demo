/*
 * @Author: plucky
 * @Date: 2023-10-23 19:34:19
 * @LastEditTime: 2024-05-17 16:03:14
 */

 #![allow(unused)]
use serde::Serialize;
use sqlx::Error;

use crate::{config::db::get_pool, query_as};


/// 菜单表
#[derive(co_orm::Crud)]
#[co_orm(rename = "sys_menu")]
#[derive(sqlx::FromRow, Debug, Serialize, Clone)]
pub struct SysMenu{
    pub id: u64,
    pub name: String, // 菜单名称
    pub menu_code: String, // 菜单编码
    pub parent_id: u64, // 父节点
    pub node_type: i8, // 节点类型 1文件夹,2页面,3按钮
    pub sort: i32, // 排序号
    pub link: String, // 页面对应的地址
    pub icon: String, // 图标
    pub level: i32, // 层次
    #[serde(skip)]
    pub path: String, // 树id的路径
    #[serde(skip)]
    pub is_delete: i8, // 是否删除 1已删除 0未删除
    #[sqlx(skip)]
    #[co_orm(skip)]
    pub children: Option<Vec<SysMenu>>, // 子节点
  
}

impl SysMenu {
    // add menu
    pub async fn add(&self) -> Result<u64, Error> {
        let pool = get_pool()?;
        let id = self.insert(pool).await?.last_insert_id();
        Ok(id)
    }

    // 获取所有菜单
    pub async fn list() -> Result<Vec<Self>, Error> {
        let pool = get_pool()?;
        let users = query_as!(SysMenu, "select * from sys_menu").fetch_all(pool).await?;

        Ok(Self::to_tree(&users,0))
    }

    // 把菜单转成树结构, parent_id=0是根节点
    pub fn to_tree(list: &Vec<Self>, parent_id: u64)->Vec<Self> {
        // 找到所有根节点
        let mut root = list.iter().filter(|x| x.parent_id == parent_id).cloned().collect::<Vec<_>>();
        for node in root.iter_mut() {
            // 递归找到所有子节点
            let children = Self::to_tree(list, node.id);
            node.children = Some(children);
            
        }

        root
        
    }
}