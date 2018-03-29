CREATE TABLE reseller.resellers (
  id int(11) NOT NULL AUTO_INCREMENT PRIMARY KEY,
  seller_id varchar(255) NOT NULL UNIQUE,
  name varchar(255),
  created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  KEY `index_resellers_on_seller_id` (`seller_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

