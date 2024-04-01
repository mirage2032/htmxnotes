ALTER SYSTEM SET max_connections = 100;
SELECT pg_reload_conf();
CREATE TABLE users
(
    id              SERIAL PRIMARY KEY,
    username        VARCHAR(50)  NOT NULL UNIQUE CHECK (LENGTH(username) >= 5),
    email           VARCHAR(254) NOT NULL UNIQUE,
    password_hash   VARCHAR  NOT NULL,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL 
    updated_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE user_groups
(
    id              SERIAL PRIMARY KEY,
    admins_ids      INTEGER[] REFERENCES users(id) ON DELETE CASCADE,
    users_ids       INTEGER[] REFERENCES users(id) ON DELETE CASCADE,
    name            VARCHAR(100) NOT NULL,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE sessions
(
    id              SERIAL PRIMARY KEY,
    user_id         INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token           VARCHAR(64) NOT NULL,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    duration        INTERVAL NOT NULL
)