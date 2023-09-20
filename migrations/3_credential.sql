CREATE TABLE IF NOT EXISTS credential (
    user_id INT NOT NULL,
    credential_type VARCHAR NOT NULL,
    credential VARCHAR NOT NULL,
    validated BOOLEAN NOT NULL,
    time_created TIMESTAMPTZ NOT NULL,
    last_updated TIMESTAMPTZ NOT NULL,
    PRIMARY KEY(user_id, credential_type),
    UNIQUE(credential)
);