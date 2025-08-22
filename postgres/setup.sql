-- https://www.postgresql.org/docs/current/ltree.html
CREATE EXTENSION IF NOT EXISTS ltree;

-- https://www.postgresql.org/docs/current/cube.html
CREATE EXTENSION IF NOT EXISTS cube;

-- https://www.postgresql.org/docs/current/citext.html
CREATE EXTENSION IF NOT EXISTS citext;

-- https://www.postgresql.org/docs/current/hstore.html
CREATE EXTENSION IF NOT EXISTS hstore;

-- https://www.postgresql.org/docs/current/sql-createtype.html
CREATE TYPE status AS ENUM ('new', 'open', 'closed');

-- https://www.postgresql.org/docs/current/rowtypes.html#ROWTYPES-DECLARING
-- CREATE TYPE inventory_item AS
-- (
--     name        TEXT,
--     supplier_id INT,
--     price       BIGINT
-- );

-- https://github.com/prisma/database-schema-examples/tree/master/postgres/basic-twitter#basic-twitter
CREATE TABLE tweet
(
    id         BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    text       TEXT        NOT NULL,
    owner_id   BIGINT
);


INSERT INTO tweet (text, owner_id) VALUES ('Hello, world!', 1);
INSERT INTO tweet (text, owner_id) VALUES ('My second tweet', 1);
INSERT INTO tweet (text, owner_id) VALUES ('This is a tweet from user 2', 2);
INSERT INTO tweet (text, owner_id) VALUES ('Another tweet from user 1', 1);

SELECT * FROM tweet;