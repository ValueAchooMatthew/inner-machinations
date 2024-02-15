-- Your SQL goes here
CREATE TABLE users(
  id INT PRIMARY KEY AUTO_INCREMENT,
  email VARCHAR(255) NOT NULL UNIQUE,
  password varchar(255)
)