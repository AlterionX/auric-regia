CREATE TABLE industry_profit_count_changes (
    id BIGSERIAL PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updater NUMERIC NOT NULL,
    target NUMERIC NOT NULL,
    alpha_united_earth_credits NUMERIC NOT NULL,
    guild_id NUMERIC NOT NULL
);

CREATE TABLE industry_profit_counts (
    user_id NUMERIC NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated TIMESTAMP WITH TIME ZONE NOT NULL,
    alpha_united_earth_credits NUMERIC NOT NULL,
    guild_id NUMERIC NOT NULL,
    id BIGSERIAL PRIMARY KEY
);

CREATE UNIQUE INDEX unique_industry_profit_count_per_guild_per_user ON industry_profit_counts (user_id, guild_id);

INSERT INTO industry_profit_count_changes (
    created,
    updater,
    target,
    alpha_united_earth_credits,
    guild_id
)
SELECT
    created,
    updater,
    target,
    total as alpha_united_earth_credits,
    guild_id
FROM tracker_count_changes
WHERE
    stat = 'industry_auec';

DELETE FROM tracker_count_changes
WHERE stat = 'industry_auec';

INSERT INTO industry_profit_counts (
    user_id,
    created,
    updated,
    alpha_united_earth_credits,
    guild_id
)
SELECT
    user_id,
    created,
    updated,
    total as alpha_united_earth_credits,
    guild_id
FROM tracker_counts
WHERE
    stat = 'industry_auec';

DELETE FROM tracker_counts
WHERE stat = 'industry_auec';
