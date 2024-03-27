CREATE TABLE vendor (
    id VARCHAR(50) PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    contact TEXT,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

