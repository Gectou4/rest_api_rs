CREATE DATABASE IF NOT EXISTS `rest_api` DEFAULT CHARACTER SET utf8 COLLATE utf8_general_ci;
USE `rest_api`;

DROP TABLE IF EXISTS `task`;
CREATE TABLE IF NOT EXISTS `task` (
  `task_id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `title` varchar(128) NOT NULL,
  `description` text NOT NULL,
  `creation_date` datetime NOT NULL,
  `status` tinyint(3) NOT NULL DEFAULT '1',
  PRIMARY KEY (`task_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 AUTO_INCREMENT=1 ;

DROP TABLE IF EXISTS `user`;
CREATE TABLE IF NOT EXISTS `user` (
  `user_id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(32) CHARACTER SET utf8 NOT NULL,
  `email` varchar(320) CHARACTER SET ascii NOT NULL COMMENT 'RFC 2821: 64+1+255 Ci',
  PRIMARY KEY (`user_id`),
  UNIQUE KEY `email` (`email`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci AUTO_INCREMENT=1 ;

DROP TABLE IF EXISTS `user_task`;
CREATE TABLE IF NOT EXISTS `user_task` (
  `user_id` int(11) unsigned NOT NULL,
  `task_id` int(11) unsigned NOT NULL,
  UNIQUE KEY `user_task` (`user_id`,`task_id`),
  CONSTRAINT `fk_ut_user` FOREIGN KEY (`user_id`) REFERENCES `user` (`user_id`) ON DELETE CASCADE,
  CONSTRAINT `fk_ut_task` FOREIGN KEY (`task_id`) REFERENCES `task` (`task_id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

INSERT INTO `rest_api`.`user` (`user_id`, `name`, `email`) VALUES ('1', 'G4', 'gectou4@gmail.com');

INSERT INTO `rest_api`.`task` (`task_id`, `title`, `description`, `creation_date`, `status`) VALUES ('1', 'Faire le cafée', 'Aller à la cafetière
mettre la tasse
Mettre le café
Mettre de l''eau si besoin
Appuyer sur le bouton "Café tiptop"
Attendre que l''eau ne goutte plus
Prendre la tasse', '2015-11-04 03:01:08', '2'), ('2', 'Boire le café', 'Prendre la tasse de café de la Task 1
Savourer', '2015-11-04 03:01:08', '1');

INSERT INTO `rest_api`.`user_task` (`user_id`, `task_id`) VALUES ('1', '1'), ('1', '2');
