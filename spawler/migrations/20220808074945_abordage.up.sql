CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username varchar(48) UNIQUE NOT NULL,
    display_name TEXT,
    gamepoints INTEGER
);

CREATE TABLE caches (
    id serial primary key,
    cache_name text not null,
    maintainer varchar(48) references users(username),
    coordinate_x point not null,
    coordinate_y point not null,
    difficulty smallint
);
