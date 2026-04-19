ALTER TABLE tracker_count_changes ADD COLUMN user_note VARCHAR(10000);

INSERT INTO tracker_count_changes (
    created,
    stat,
    guild_id,
    updater,
    target,
    total,
    user_note
)
SELECT
    created,
    'event_participation' AS stat,
    guild_id,
    updater,
    target,
    total,
    user_note
FROM tracker_count_changes;

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
    'event_participation' AS stat,
    guild_id,
    user_id,
    total
FROM tracker_counts;

DROP INDEX unique_event_participation_count_per_guild_per_user;
DROP TABLE event_participation_counts, event_participation_count_changes;
