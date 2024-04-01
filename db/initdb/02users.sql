CREATE TABLE users
(
    id              SERIAL PRIMARY KEY,
    username        VARCHAR(50)  NOT NULL UNIQUE CHECK (LENGTH(username) >= 5),
    email           VARCHAR(254) NOT NULL UNIQUE,
    password_hash   VARCHAR(100)  NOT NULL,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER users_update_trigger
    BEFORE INSERT ON users
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE user_groups
(
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER user_groups_update_trigger
    BEFORE INSERT  ON user_groups
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TYPE user_role AS ENUM ('admin', 'normal');

CREATE TABLE user_group_members
(
    id              SERIAL PRIMARY KEY,
    user_id         INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    group_id        INTEGER NOT NULL REFERENCES user_groups(id) ON DELETE CASCADE,
    role            user_role NOT NULL,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT unique_user_note_combination_ugm UNIQUE (user_id, group_id)
);

CREATE TABLE sessions
(
    id              SERIAL PRIMARY KEY,
    user_id         INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token           VARCHAR(64) NOT NULL,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    duration        INTERVAL NOT NULL
);