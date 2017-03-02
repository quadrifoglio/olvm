CREATE DATABASE IF NOT EXISTS olvm;
USE olvm;

CREATE TABLE IF NOT EXISTS node (
	id INTEGER NOT NULL UNIQUE PRIMARY KEY,
	host VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS image (
	id INTEGER NOT NULL UNIQUE PRIMARY KEY AUTO_INCREMENT,
	ref_node INTEGER NOT NULL REFERENCES node(id),
	name VARCHAR(255) NOT NULL,
	file VARCHAR(255) NOT NULL
);

REPLACE INTO node (id, host) VALUES (1, '127.0.0.1');
