
-- insert into users (name, password) values ('plucky', '123456');


-- 菜单权限控制
CREATE DATABASE IF NOT EXISTS menu_auth default charset utf8mb4 COLLATE utf8mb4_unicode_ci;

use menu_auth;

CREATE TABLE users (
  id bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  name varchar(128) NOT NULL DEFAULT '' COMMENT '姓名',
  password varchar(128) NOT NULL DEFAULT '' COMMENT '密码',
  is_delete tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否删除 1:已删除;0:未删除',
  created_at datetime NULL DEFAULT now(),
  updated_at datetime NULL DEFAULT now(),

  PRIMARY KEY (id),
  KEY idx_name (name) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户表';

CREATE TABLE sys_user_role (
  id bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  user_id bigint(20) unsigned NOT NULL COMMENT '用户ID',
  role_id bigint(20) unsigned NOT NULL COMMENT '角色ID',
  PRIMARY KEY (id),
  KEY idx_user_id (user_id) USING BTREE,
  KEY idx_role_id (role_id) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户角色表';

CREATE TABLE sys_role (
  id bigint(20) unsigned NOT NULL AUTO_INCREMENT ,
  code varchar(100) NOT NULL DEFAULT '' COMMENT '编码',
  name varchar(100) NOT NULL DEFAULT '' COMMENT '名称',
  is_delete tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否删除 1:已删除;0:未删除',
  PRIMARY KEY (id),
  KEY idx_code (code) USING BTREE,
  KEY idx_name (name) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='角色表';


CREATE TABLE sys_role_menu (
  id bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  role_id bigint(20) unsigned NOT NULL COMMENT '角色ID',
  menu_id bigint(20) unsigned NOT NULL COMMENT '菜单ID',
  PRIMARY KEY (id),
  KEY idx_role_id (role_id) USING BTREE,
  KEY idx_menu_id (menu_id) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='角色菜单关系表';


CREATE TABLE sys_menu (
  id bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  name varchar(50) NOT NULL DEFAULT '' COMMENT '名称',
  menu_code varchar(100) NOT NULL DEFAULT '' COMMENT '菜单编码',
  parensys_id bigint(20) unsigned DEFAULT 0 COMMENT '父节点',
  node_type tinyint(1) NOT NULL DEFAULT '1' COMMENT '节点类型:1文件夹,2页面,3按钮',
  sort int(11) NOT NULL DEFAULT '1' COMMENT '排序号',
  link varchar(200) DEFAULT '' COMMENT '页面对应的地址',
  icon varchar(100) DEFAULT '' COMMENT '图标地址',
  level int(11) NOT NULL DEFAULT '0' COMMENT '层次',
  path varchar(1500) DEFAULT '' COMMENT '树id的路径:整个层次上的路径id,逗号分隔,想要找父节点特别快',
  is_delete tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否删除 1:已删除;0:未删除',
  PRIMARY KEY (id) USING BTREE,
  KEY idx_parensys_id (parensys_id) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='菜单表';

-- SELECT auto_increment FROM information_schema.tables where table_schema="dbName" and table_name="tableName";


CREATE TABLE IF NOT EXISTS casbin_rule (
    id BIGINT unsigned NOT NULL AUTO_INCREMENT,
    ptype VARCHAR(12) NOT NULL,
    v0 VARCHAR(128) NOT NULL,
    v1 VARCHAR(128) NOT NULL,
    v2 VARCHAR(128) NOT NULL,
    v3 VARCHAR(128) NOT NULL,
    v4 VARCHAR(128) NOT NULL,
    v5 VARCHAR(128) NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT unique_key_sqlx_adapter UNIQUE(ptype, v0, v1, v2, v3, v4, v5),
    KEY idx_name (v0) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
 