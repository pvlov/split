CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	profile_picture VARCHAR(255) NOT NULL DEFAULT 'penguin',
    username VARCHAR(255) UNIQUE NOT NULL,
	hashed_password BYTEA NOT NULL,
	description VARCHAR(255) NOT NULL DEFAULT 'No description available',	
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS groups (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	name VARCHAR(255) NOT NULL,
	description VARCHAR(255) NOT NULL DEFAULT 'No description available',
	created_at TIMESTAMPTZ DEFAULT NOW(),
	updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS group_membership (
	user_id UUID NOT NULL,
	group_id UUID NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
	PRIMARY KEY (user_id, group_id)
);

CREATE TABLE IF NOT EXISTS payment (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	amount INTEGER NOT NULL CHECK (amount > 0), -- in euro cents
	group_id UUID NOT NULL,
	from_id UUID NOT NULL,
	to_id UUID NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
	FOREIGN KEY (from_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (to_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS debt (
	payment_id UUID NOT NULL,
	from_id UUID NOT NULL,
	to_id UUID NOT NULL,
	amount INTEGER NOT NULL CHECK (amount > 0), -- in euro cents
	FOREIGN KEY (payment_id) REFERENCES payment(id), -- even if the payment is gone, we want to keep the info that there is debt
	FOREIGN KEY (from_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (to_id) REFERENCES users(id) ON DELETE CASCADE
);
