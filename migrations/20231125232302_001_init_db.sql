-- 001_init_db.sql
-- sqlx-migrate:up
CREATE TABLE guilds (
    guild_id bigint PRIMARY KEY,
    prefix text

);
