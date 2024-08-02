CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) UNIQUE NOT NULL,
	hashed_password BYTEA NOT NULL,
	payment_description VARCHAR(255) NOT NULL DEFAULT 'No payment description available',	
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);

CREATE TABLE IF NOT EXISTS groups (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS user_groups (
    user_id UUID REFERENCES users(id) ON UPDATE CASCADE ON DELETE CASCADE, 
    group_id UUID REFERENCES groups(id) ON UPDATE CASCADE,
	CONSTRAINT bill_product_pkey PRIMARY KEY (user_id, group_id)
);

