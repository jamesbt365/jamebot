-- 001_init_db.sql
-- sqlx-migrate:up
CREATE TABLE guilds (
    guild_id bigint PRIMARY KEY,
    prefix text,

    -- Guild Moderation - Media:
    media_del_save boolean,
    media_save_total_limit int,
    media_save_limit smallint

);

