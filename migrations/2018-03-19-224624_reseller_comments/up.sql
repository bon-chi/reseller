CREATE TABLE reseller.reseller_comments (
  id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  reseller_id int(11) NOT NULL,
  comment text NOT NULL,
  user_name varchar(255),
  pass varchar(255) NOT NULL,
  created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  KEY `index_reseller_comments_on_reseller_id` (`reseller_id`),
  CONSTRAINT `fk_reseller_id` FOREIGN KEY (`reseller_id`) REFERENCES `resellers` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
