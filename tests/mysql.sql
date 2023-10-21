use hello;
CREATE TABLE `users` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(128) NOT NULL,
  `password` varchar(128) NOT NULL,
  `created_at` datetime NULL DEFAULT now(),
  `updated_at` datetime NULL DEFAULT now(),

  UNIQUE KEY (`name`),
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ALTER table users add UNIQUE index (name);

-- insert into users (name, password, created_at, updated_at) values ('plucky', '123456', now(), now());