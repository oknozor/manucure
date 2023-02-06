-- Add migration script here
CREATE TABLE users(
  id BIGSERIAL PRIMARY KEY,
  username VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL
);

CREATE TABLE article (
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT NOT NULL REFERENCES users(id),
  url TEXT NOT NULL,
  title VARCHAR(255) NOT NULL,
  text TEXT NOT NULL,
  content TEXT NOT NULL
);
