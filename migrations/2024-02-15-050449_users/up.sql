-- Your SQL goes here
CREATE TABLE users(
  id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
  email VARCHAR(255) NOT NULL UNIQUE,
  password varchar(255) NOT NULL,
  verified TINYINT(1) NOT NULL,
  code varchar(6)
);

CREATE TABLE saved_automata(
  id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
  u_id INT NOT NULL,
  name varchar(255) NOT NULL UNIQUE,
  FOREIGN KEY (u_id) REFERENCES users(id)
);

CREATE TABLE saved_states (
  id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
  automata_id INT NOT NULL, 
  FOREIGN KEY (automata_id) REFERENCES saved_automata(id), 
  position VARCHAR(255) NOT NULL,
  connected_state INT,
  FOREIGN KEY (connected_state) REFERENCES saved_automata(id),
  connection_character VARCHAR(1) NOT NULL,
  is_start TINYINT(1) NOT NULL,
  is_final TINYINT(1) NOT NULL
);

CREATE TABLE saved_connections (
  id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
  automata_id INT NOT NULL, 
  FOREIGN KEY (automata_id) REFERENCES saved_automata(id), 
  start_coords VARCHAR(255) NOT NULL, 
  control_point_one VARCHAR(255) NOT NULL, 
  control_point_two VARCHAR(255) NOT NULL, 
  end_coords VARCHAR(255) NOT NULL
);

ALTER TABLE users ALTER verified SET DEFAULT 0;