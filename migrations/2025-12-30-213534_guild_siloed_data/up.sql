ALTER TABLE naval_victory_count_changes
    ADD COLUMN guild_id NUMERIC NOT NULL DEFAULT 1014283099028324503;
ALTER TABLE naval_victory_counts
    ADD COLUMN guild_id NUMERIC NOT NULL DEFAULT 1014283099028324503;
ALTER TABLE legion_kill_count_changes
    ADD COLUMN guild_id NUMERIC NOT NULL DEFAULT 1014283099028324503;
ALTER TABLE legion_kill_counts
    ADD COLUMN guild_id NUMERIC NOT NULL DEFAULT 1014283099028324503;
ALTER TABLE industry_profit_count_changes
    ADD COLUMN guild_id NUMERIC NOT NULL DEFAULT 1014283099028324503;
ALTER TABLE industry_profit_counts
    ADD COLUMN guild_id NUMERIC NOT NULL DEFAULT 1014283099028324503;
ALTER TABLE event_participation_count_changes
    ADD COLUMN guild_id NUMERIC NOT NULL DEFAULT 1014283099028324503;
ALTER TABLE event_participation_counts
    ADD COLUMN guild_id NUMERIC NOT NULL DEFAULT 1014283099028324503;
ALTER TABLE naval_tackle_assist_count_changes
    ADD COLUMN guild_id NUMERIC NOT NULL DEFAULT 1014283099028324503;
ALTER TABLE naval_tackle_assist_counts
    ADD COLUMN guild_id NUMERIC NOT NULL DEFAULT 1014283099028324503;
ALTER TABLE monthly_goals
    ADD COLUMN guild_id NUMERIC NOT NULL DEFAULT 1014283099028324503;

ALTER TABLE naval_victory_count_changes
    ALTER COLUMN guild_id DROP DEFAULT;
ALTER TABLE naval_victory_counts
    ALTER COLUMN guild_id DROP DEFAULT;
ALTER TABLE legion_kill_count_changes
    ALTER COLUMN guild_id DROP DEFAULT;
ALTER TABLE legion_kill_counts
    ALTER COLUMN guild_id DROP DEFAULT;
ALTER TABLE industry_profit_count_changes
    ALTER COLUMN guild_id DROP DEFAULT;
ALTER TABLE industry_profit_counts
    ALTER COLUMN guild_id DROP DEFAULT;
ALTER TABLE event_participation_count_changes
    ALTER COLUMN guild_id DROP DEFAULT;
ALTER TABLE event_participation_counts
    ALTER COLUMN guild_id DROP DEFAULT;
ALTER TABLE naval_tackle_assist_count_changes
    ALTER COLUMN guild_id DROP DEFAULT;
ALTER TABLE naval_tackle_assist_counts
    ALTER COLUMN guild_id DROP DEFAULT;
ALTER TABLE monthly_goals
    ALTER COLUMN guild_id DROP DEFAULT;

-- Need to reindex tables that are based off of user
ALTER TABLE naval_victory_counts
    DROP CONSTRAINT naval_victory_counts_pkey;
ALTER TABLE naval_victory_counts
    RENAME COLUMN id TO user_id;
ALTER TABLE naval_victory_counts
    ADD COLUMN id BIGSERIAL NOT NULL,
    ADD PRIMARY KEY (id);
ALTER TABLE legion_kill_counts
    DROP CONSTRAINT legion_kill_counts_pkey;
ALTER TABLE legion_kill_counts
    RENAME COLUMN id TO user_id;
ALTER TABLE legion_kill_counts
    ADD COLUMN id BIGSERIAL NOT NULL,
    ADD PRIMARY KEY (id);
ALTER TABLE industry_profit_counts
    DROP CONSTRAINT industry_profit_counts_pkey;
ALTER TABLE industry_profit_counts
    RENAME COLUMN id TO user_id;
ALTER TABLE industry_profit_counts
    ADD COLUMN id BIGSERIAL NOT NULL,
    ADD PRIMARY KEY (id);
ALTER TABLE event_participation_counts
    DROP CONSTRAINT event_participation_counts_pkey;
ALTER TABLE event_participation_counts
    RENAME COLUMN id TO user_id;
ALTER TABLE event_participation_counts
    ADD COLUMN id BIGSERIAL NOT NULL,
    ADD PRIMARY KEY (id);
ALTER TABLE naval_tackle_assist_counts
    DROP CONSTRAINT naval_tackle_assist_counts_pkey;
ALTER TABLE naval_tackle_assist_counts
    RENAME COLUMN id TO user_id;
ALTER TABLE naval_tackle_assist_counts
    ADD COLUMN id BIGSERIAL NOT NULL,
    ADD PRIMARY KEY (id);

DROP INDEX monthly_goal_unique_active_shortname;
CREATE UNIQUE INDEX monthly_goal_unique_active_shortname ON monthly_goals (shortname, guild_id) WHERE disabled IS NULL;

-- Now that we don't have the primary key constraint, we still need to keep this unique.
CREATE UNIQUE INDEX unique_naval_victory_count_per_guild_per_user ON naval_victory_counts (user_id, guild_id);
CREATE UNIQUE INDEX unique_legion_kill_count_per_guild_per_user ON legion_kill_counts (user_id, guild_id);
CREATE UNIQUE INDEX unique_industry_profit_count_per_guild_per_user ON industry_profit_counts (user_id, guild_id);
CREATE UNIQUE INDEX unique_event_participation_count_per_guild_per_user ON event_participation_counts (user_id, guild_id);
CREATE UNIQUE INDEX unique_naval_tackle_assist_count_per_guild_per_user ON naval_tackle_assist_counts (user_id, guild_id);
