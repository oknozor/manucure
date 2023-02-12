-- Add migration script here
CREATE TABLE users
(
    id       BIGSERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    email    TEXT NOT NULL
);

CREATE TABLE article
(
    id       BIGSERIAL PRIMARY KEY,
    user_id  BIGINT                   NOT NULL REFERENCES users (id),
    url      TEXT                     NOT NULL,
    title    TEXT                     NOT NULL,
    text     TEXT                     NOT NULL,
    content  TEXT                     NOT NULL,
    starred  BOOLEAN                  NOT NULL DEFAULT false,
    archived BOOLEAN                  NOT NULL DEFAULT false,
    created  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);


CREATE TABLE tag
(
    id      BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users (id),
    name    TEXT   NOT NULL,
    UNIQUE (user_id, name)
);

CREATE TABLE article_tag
(
    tag_id     BIGINT NOT NULL REFERENCES tag (id),
    article_id BIGINT NOT NULL REFERENCES article (id) ON DELETE CASCADE,
    PRIMARY KEY (tag_id, article_id)
)
