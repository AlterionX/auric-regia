CREATE TABLE tracker_count_changes (
    id BIGSERIAL PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    stat VARCHAR(500) NOT NULL,
    guild_id NUMERIC NOT NULL,
    updater NUMERIC NOT NULL,
    target NUMERIC NOT NULL,
    total NUMERIC NOT NULL
);

CREATE TABLE tracker_counts (
    id BIGSERIAL PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    stat VARCHAR(100) NOT NULL,
    guild_id NUMERIC NOT NULL,
    user_id NUMERIC NOT NULL,
    total NUMERIC NOT NULL
);

CREATE UNIQUE INDEX unique_tracker_count_per_stat_per_guild_per_user ON tracker_counts (stat, guild_id, user_id);
