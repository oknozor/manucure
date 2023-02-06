-- Add migration script here
CREATE TABLE users(
  id BIGSERIAL,
  username VARCHAR(255),
  email VARCHAR(255)
);

CREATE TABLE article (
  id BIGSERIAL,
  url TEXT NOT NULL,
  title VARCHAR(255) NOT NULL,
  text TEXT NOT NULL,
  content TEXT NOT NULL
);
