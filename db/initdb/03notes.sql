CREATE TABLE notes
(
    id              SERIAL PRIMARY KEY,
    title           VARCHAR(100) NOT NULL,
    content         TEXT NOT NULL,
    public          BOOLEAN NOT NULL,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER notes_update_trigger
    BEFORE INSERT ON notes
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TYPE note_user_role AS ENUM ('admin', 'writer', 'reader');

CREATE TABLE note_permissions
(
    id              SERIAL PRIMARY KEY,
    note_id         INTEGER NOT NULL REFERENCES notes(id),
    user_id         INTEGER NOT NULL REFERENCES users(id),
    group_id        INTEGER NOT NULL REFERENCES user_groups(id),
    permission      note_user_role NOT NULL,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT check_note_user_exclusive_np CHECK (
        (user_id IS NOT NULL AND group_id IS NULL) OR
        (user_id IS NULL AND group_id IS NOT NULL)
        ),
    CONSTRAINT unique_user_note_combination_np UNIQUE (note_id, user_id),
    CONSTRAINT unique_group_note_combination_np UNIQUE (note_id, group_id)
);

CREATE TABLE note_groups
(
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    description     TEXT,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER note_group_update_trigger
    BEFORE INSERT ON note_groups
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE note_group_permissions
(
    id              SERIAL PRIMARY KEY,
    note_id         INTEGER NOT NULL REFERENCES notes(id),
    user_id         INTEGER NOT NULL REFERENCES users(id),
    group_id        INTEGER NOT NULL REFERENCES user_groups(id),
    permission      note_user_role NOT NULL,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT check_note_user_exclusive_ngp CHECK (
        (user_id IS NOT NULL AND group_id IS NULL) OR
        (user_id IS NULL AND group_id IS NOT NULL)
        ),
    CONSTRAINT unique_user_note_combination_ngp UNIQUE (note_id, user_id),
    CONSTRAINT unique_group_note_combination_ngp UNIQUE (note_id, group_id)
);

CREATE TABLE note_group_members
(
    id              SERIAL PRIMARY KEY,
    note_id         INTEGER NOT NULL REFERENCES notes(id),
    group_id        INTEGER NOT NULL REFERENCES note_groups(id),
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT unique_note_group_combination_ngm UNIQUE (note_id, group_id)
);