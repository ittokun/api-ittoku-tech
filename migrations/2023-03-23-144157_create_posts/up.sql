CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "posts" (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
	title VARCHAR NOT NULL,
	body TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
	updated_at TIMESTAMP
);
