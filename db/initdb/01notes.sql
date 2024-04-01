CREATE TABLE notes
(
    id              SERIAL PRIMARY KEY,
    title           VARCHAR(100) NOT NULL,
    content         TEXT NOT NULL,
    public          BOOLEAN NOT NULL,
    maintainers_ids INTEGER[] REFERENCES users(id) ON DELETE CASCADE,
    readers_ids     INTEGER[] REFERENCES users(id) ON DELETE CASCADE,
    writers_ids     INTEGER[] REFERENCES users(id) ON DELETE CASCADE,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE note_groups
(
    id              SERIAL PRIMARY KEY,
    notes_ids       INTEGER[] REFERENCES notes(id) ON DELETE CASCADE,
    name            VARCHAR(100) NOT NULL,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL ON UPDATE CURRENT_TIMESTAMP
);