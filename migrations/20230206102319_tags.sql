CREATE TABLE tag(
  id BIGSERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  color VARCHAR(6) NOT NULL DEFAULT 'b4eeb4'
);

CREATE TABLE article_tag(
  user_id BIGINT NOT NULL REFERENCES users(id),
  tag_id BIGINT NOT NULL REFERENCES tag(id),
  article_id BIGINT NOT NULL REFERENCES article(id),
  PRIMARY KEY(tag_id, article_id, user_id)
)
