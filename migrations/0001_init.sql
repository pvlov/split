CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(64) NOT NULL,
    email VARCHAR(64) NOT NULL
);

CREATE TABLE IF NOT EXISTS groups (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	name VARCHAR(64) NOT NULL
);

CREATE TABLE IF NOT EXISTS user_groups (
    user_id UUID,
    group_id UUID,
    PRIMARY KEY (user_id, group_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (group_id) REFERENCES groups(id)
);
