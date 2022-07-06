# rust + axum + sqlx + redis + jwt

### 表结构

```
CREATE TABLE `short_links` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `url` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=0 DEFAULT CHARSET=utf8;
```

### 编译问题

sqlx-adapter 用到macro：query! 等，第一次build要在终端配置环境变量DATABASE_URL，再cargo build

```
export DATABASE_URL=mysql://root:newpassword@192.168.1.199:3306/casbin
```
