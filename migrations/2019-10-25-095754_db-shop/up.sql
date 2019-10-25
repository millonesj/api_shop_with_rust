-- Your SQL goes here
CREATE TABLE products (
  id INT(11) PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(100) NOT NULL,
  price DECIMAL(5,2) NOT NULL,
  currency CHAR(3) NOT NULL,
  owner INT(11) NOT NULL
)
