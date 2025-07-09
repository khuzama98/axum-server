-- Add profile fields to users table
ALTER TABLE users 
ADD COLUMN first_name VARCHAR(100),
ADD COLUMN last_name VARCHAR(100),
ADD COLUMN bio TEXT,
ADD COLUMN avatar_url VARCHAR(500),
ADD COLUMN is_active BOOLEAN DEFAULT TRUE;

-- Create indexes for performance
CREATE INDEX idx_users_full_name ON users(first_name, last_name);
CREATE INDEX idx_users_active ON users(is_active);