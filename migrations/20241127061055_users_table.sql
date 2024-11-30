-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; -- Enables UUID generation functions

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(), -- UUID as primary key, auto-generated
    username VARCHAR(50) NOT NULL UNIQUE,           -- Unique username
    email VARCHAR(100) NOT NULL UNIQUE,             -- Unique email address
    password_hash VARCHAR(255) NOT NULL,            -- Hashed password for security
    first_name VARCHAR(50),                         -- Optional first name
    last_name VARCHAR(50),                          -- Optional last name
    date_of_birth DATE,                             -- Optional date of birth
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Automatically sets the creation timestamp
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Automatically updates on modification
    is_active BOOLEAN DEFAULT TRUE                  -- Indicates if the user account is active
);
