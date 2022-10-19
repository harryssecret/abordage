-- Add up migration script here
CREATE EXTENSION Postgis;

CREATE TYPE cache_status AS ENUM ('Archived', 'Maintenance', 'Active');

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username varchar(48) UNIQUE NOT NULL,
    pirate_password varchar(512) NOT NULL,
    display_name TEXT,
    post_count int default 0,
    gamepoints INT NOT NULL DEFAULT 0
);

CREATE TABLE caches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cache_name TEXT NOT NULL,
    cache_description jsonb,
    maintainer VARCHAR(48) NOT NULL REFERENCES users(username),
    cache_location geography(POINT, 4326) not null,
    coordinate_y point not null,
    difficulty smallint not null CHECK (difficulty >= 0 and difficulty < 10),
    archived_state cache_status not null default 'Active'
);

