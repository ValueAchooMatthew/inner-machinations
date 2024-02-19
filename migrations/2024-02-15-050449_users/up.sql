-- Your SQL goes here
CREATE TABLE users(
  id INT PRIMARY KEY AUTO_INCREMENT,
  email VARCHAR(255) NOT NULL UNIQUE,
  password varchar(255) NOT NULL,
  verified TINYINT(1) NOT NULL 
);

ALTER TABLE users ALTER verified SET DEFAULT 0;