-- Your SQL goes here
CREATE TABLE distilleries (
  id integer AUTO_INCREMENT PRIMARY KEY,
  whisky_type varchar(255) NOT NULL,
  region varchar(255),
  name varchar(255) NOT NULL,
  name_ja varchar(255) NOT NULL,
  owner varchar(255) NOT NULL,
  owner_ja varchar(255) NOT NULL
)