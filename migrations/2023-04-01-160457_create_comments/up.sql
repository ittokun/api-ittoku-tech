CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "comments" (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
	body TEXT NOT NULL,
	post_id UUID REFERENCES posts(id) NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
	updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);
