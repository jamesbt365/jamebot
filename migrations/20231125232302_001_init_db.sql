-- 001_init_db.sql
-- sqlx-migrate:up
CREATE TABLE guilds (
    guild_id bigint PRIMARY KEY,
    prefix text

);

CREATE TABLE guild_data (
    guild_id INT PRIMARY KEY,
    elevation_info jsob
);
