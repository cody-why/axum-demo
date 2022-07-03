<!--
 * @Author: plucky
 * @Date: 2022-06-27 16:36:01
 * @LastEditTime: 2022-07-01 14:57:04
 * @Description: 
-->
## 表结构
```
CREATE TABLE `short_links` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `url` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=0 DEFAULT CHARSET=utf8;
```

## postman
```
/api/shortlink/:id

/api/create_shortlink
{"url":"http://hello.com"}

/api/delete_shortlink
{"id":1}

/api/update_shortlink
{"id":1,url:"http://hello.com/h"}

```