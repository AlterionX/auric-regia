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
    'ground_kill' AS stat,
    guild_id,
    updater,
    target,
    kills
FROM legion_kill_count_changes;

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
    'ground_kill' AS stat,
    guild_id,
    user_id,
    kills
FROM legion_kill_counts;


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
    'naval_victory' AS stat,
    guild_id,
    updater,
    target,
    victory_fourths
FROM naval_victory_count_changes;

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
    'naval_victory' AS stat,
    guild_id,
    user_id,
    victory_fourths
FROM naval_victory_counts;


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
    'naval_tackle_assist' AS stat,
    guild_id,
    updater,
    target,
    tackle_assists
FROM naval_tackle_assist_count_changes;

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
    'naval_tackle_assist' AS stat,
    guild_id,
    user_id,
    tackle_assists
FROM naval_tackle_assist_counts;


DROP INDEX unique_legion_kill_count_per_guild_per_user, unique_naval_victory_count_per_guild_per_user, unique_naval_tackle_assist_count_per_guild_per_user;
DROP TABLE legion_kill_count_changes, legion_kill_counts, naval_victory_count_changes, naval_victory_counts, naval_tackle_assist_count_changes, naval_tackle_assist_counts;
