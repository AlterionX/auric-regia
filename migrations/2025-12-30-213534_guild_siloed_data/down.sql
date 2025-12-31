DROP INDEX unique_naval_victory_count_per_guild_per_user;
DROP INDEX unique_legion_kill_count_per_guild_per_user;
DROP INDEX unique_industry_profit_count_per_guild_per_user;
DROP INDEX unique_event_participation_count_per_guild_per_user;
DROP INDEX unique_naval_tackle_assist_count_per_guild_per_user;

-- And now we need to reindex back
ALTER TABLE naval_victory_counts
    DROP CONSTRAINT naval_victory_counts_pkey,
    DROP COLUMN id,
    ADD PRIMARY KEY (user_id);
ALTER TABLE naval_victory_counts
    RENAME COLUMN user_id TO id;
ALTER TABLE legion_kill_counts
    DROP CONSTRAINT legion_kill_counts_pkey,
    DROP COLUMN id,
    ADD PRIMARY KEY (user_id);
ALTER TABLE legion_kill_counts
    RENAME COLUMN user_id TO id;
ALTER TABLE industry_profit_counts
    DROP CONSTRAINT industry_profit_counts_pkey,
    DROP COLUMN id,
    ADD PRIMARY KEY (user_id);
ALTER TABLE industry_profit_counts
    RENAME COLUMN user_id TO id;
ALTER TABLE event_participation_counts
    DROP CONSTRAINT event_participation_counts_pkey,
    DROP COLUMN id,
    ADD PRIMARY KEY (user_id);
ALTER TABLE event_participation_counts
    RENAME COLUMN user_id TO id;
ALTER TABLE naval_tackle_assist_counts
    DROP CONSTRAINT naval_tackle_assist_counts_pkey,
    DROP COLUMN id,
    ADD PRIMARY KEY (user_id);
ALTER TABLE naval_tackle_assist_counts
    RENAME COLUMN user_id TO id;

-- need to drop these so that the new constraints (unique active shortname) don't apply and
-- data isn't corrupted
DELETE FROM
    naval_victory_count_changes
WHERE guild_id <> 1014283099028324503;
DELETE FROM
    naval_victory_counts
WHERE guild_id <> 1014283099028324503;
DELETE FROM
    legion_kill_count_changes
WHERE guild_id <> 1014283099028324503;
DELETE FROM
    legion_kill_counts
WHERE guild_id <> 1014283099028324503;
DELETE FROM
    industry_profit_count_changes
WHERE guild_id <> 1014283099028324503;
DELETE FROM
    industry_profit_counts
WHERE guild_id <> 1014283099028324503;
DELETE FROM
    event_participation_count_changes
WHERE guild_id <> 1014283099028324503;
DELETE FROM
    event_participation_counts
WHERE guild_id <> 1014283099028324503;
DELETE FROM
    naval_tackle_assist_count_changes
WHERE guild_id <> 1014283099028324503;
DELETE FROM
    naval_tackle_assist_counts
WHERE guild_id <> 1014283099028324503;
DELETE FROM
    monthly_goals
WHERE guild_id <> 1014283099028324503;

DROP INDEX monthly_goal_unique_active_shortname;
CREATE UNIQUE INDEX monthly_goal_unique_active_shortname ON monthly_goals (shortname) WHERE disabled IS NULL;

ALTER TABLE naval_victory_count_changes
    DROP COLUMN guild_id;
ALTER TABLE naval_victory_counts
    DROP COLUMN guild_id;
ALTER TABLE legion_kill_count_changes
    DROP COLUMN guild_id;
ALTER TABLE legion_kill_counts
    DROP COLUMN guild_id;
ALTER TABLE industry_profit_count_changes
    DROP COLUMN guild_id;
ALTER TABLE industry_profit_counts
    DROP COLUMN guild_id;
ALTER TABLE event_participation_count_changes
    DROP COLUMN guild_id;
ALTER TABLE event_participation_counts
    DROP COLUMN guild_id;
ALTER TABLE naval_tackle_assist_count_changes
    DROP COLUMN guild_id;
ALTER TABLE naval_tackle_assist_counts
    DROP COLUMN guild_id;
ALTER TABLE monthly_goals
    DROP COLUMN guild_id;
