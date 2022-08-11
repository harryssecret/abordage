-- Add up migration script here
CREATE TABLE caches {
    id serial primary key,
    cache_name text not null,
    coordinate_x point not null,
    coordinate_y point not null,
    difficulty smallint,
    maintainer varchar(48) references users(username)
}

CREATE TABLE users {
    id SERIAL PRIMARY KEY,
    username varchar(48) NOT NULL,
    display_name TEXT,
    gamepoints INTEGER
}