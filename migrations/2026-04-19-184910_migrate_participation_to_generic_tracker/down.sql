CREATE TABLE event_participation_count_changes (
    id BIGSERIAL PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updater NUMERIC NOT NULL,
    target NUMERIC NOT NULL,
    event_participation NUMERIC NOT NULL,
    user_note VARCHAR(10000),
    guild_id NUMERIC NOT NULL
);

CREATE TABLE event_participation_counts (
    user_id NUMERIC NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated TIMESTAMP WITH TIME ZONE NOT NULL,
    event_participation NUMERIC NOT NULL,
    guild_id NUMERIC NOT NULL,
    id BIGINT PRIMARY KEY
);

CREATE UNIQUE INDEX unique_event_participation_count_per_guild_per_user ON event_participation_counts (user_id, guild_id);

INSERT INTO event_participation_count_changes (
    created,
    updater,
    target,
    event_participation,
    user_note,
    guild_id
)
SELECT
    created,
    updater,
    target,
    total as event_participation,
    user_note,
    guild_id
FROM tracker_count_changes
WHERE
    stat = 'event_participation';

INSERT INTO event_participation_counts (
    user_id,
    created,
    updated,
    event_participation,
    guild_id
)
SELECT
    user_id,
    created,
    updated,
    total as event_participation,
    guild_id
FROM tracker_counts
WHERE
    stat = 'event_participation';

ALTER TABLE tracker_count_changes DROP COLUMN user_note;
