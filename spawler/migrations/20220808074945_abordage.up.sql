CREATE TYPE cache_status AS ENUM ('archived', 'maintenance', 'active');

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
    maintainer varchar(48) not null references users(username),
    coordinate_x point not null,
    coordinate_y point not null,
    difficulty smallint not null CHECK (difficulty > 0 and difficulty < 10),
    archived_state cache_status not null default 'active'
);
