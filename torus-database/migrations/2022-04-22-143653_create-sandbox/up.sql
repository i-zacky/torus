CREATE TABLE sandbox (
    id VARCHAR(26) PRIMARY KEY,
    name VARCHAR(50),
    birthday DATE,
    height INTEGER,
    weight NUMERIC(4, 1),
    enabled BOOLEAN,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)
