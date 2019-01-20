-- Your SQL goes here

CREATE SEQUENCE heroes_seq;

CREATE TABLE heroes (
  id INT PRIMARY KEY DEFAULT NEXTVAL ('heroes_seq'),
  name VARCHAR(60) NOT NULL,
  identity VARCHAR(60) NOT NULL,
  hometown VARCHAR(60) NOT NULL,
  age INT NOT NULL
)