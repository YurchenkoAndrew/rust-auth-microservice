CREATE TABLE oauth_access_tokens (
    id VARCHAR(255) PRIMARY KEY,
    user_id BIGINT NOT NULL,
    client_id BIGINT NOT NULL,
    name VARCHAR(255),
    scopes VARCHAR(255),
    revoked BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT current_timestamp NOT NULL,
    expires_at TIMESTAMPTZ
);