-- Your SQL goes here
CREATE TABLE posts (
  id integer AUTO_INCREMENT PRIMARY KEY,
  title varchar(255) NOT NULL,
  body text NOT NULL,
  published bool NOT NULL DEFAULT false
)
