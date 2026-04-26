INSERT INTO tracker_count_changes (
    created,
    stat,
    guild_id,
    updater,
    target,
    total
)
SELECT
    created,
    'industry_auec' AS stat,
    guild_id,
    updater,
    target,
    alpha_united_earth_credits
FROM industry_profit_count_changes;

INSERT INTO tracker_counts (
    created,
    updated,
    stat,
    guild_id,
    user_id,
    total
)
SELECT
    created,
    updated,
    'industry_auec' AS stat,
    guild_id,
    user_id,
    alpha_united_earth_credits
FROM industry_profit_counts;

DROP INDEX unique_industry_profit_count_per_guild_per_user;
DROP TABLE industry_profit_count_changes, industry_profit_counts;
