CREATE EXTENSION Postgis;

CREATE TYPE cache_status AS ENUM ('Archived', 'Maintenance', 'Active');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username varchar(48) UNIQUE NOT NULL,
    pirate_password varchar(512) NOT NULL,
    display_name TEXT,
    post_count int default 0,
    gamepoints INT NOT NULL DEFAULT 0
);

CREATE TABLE caches (
    id serial primary key,
    cache_name text not null,
    cache_description jsonb,
    maintainer varchar(48) not null references users(username),
    cache_location geography(POINT, 4326) not null,
    coordinate_y point not null,
    difficulty smallint not null CHECK (difficulty >= 0 and difficulty < 10),
    archived_state cache_status not null default 'Active'
);
