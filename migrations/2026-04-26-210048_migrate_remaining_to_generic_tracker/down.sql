CREATE TABLE legion_kill_count_changes (
    id BIGSERIAL PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updater NUMERIC NOT NULL,
    target NUMERIC NOT NULL,
    kills NUMERIC NOT NULL,
    guild_id NUMERIC NOT NULL
);

CREATE TABLE legion_kill_counts (
    user_id NUMERIC NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated TIMESTAMP WITH TIME ZONE NOT NULL,
    kills NUMERIC NOT NULL,
    guild_id NUMERIC NOT NULL,
    id BIGSERIAL PRIMARY KEY
);

CREATE TABLE naval_victory_count_changes (
    id BIGSERIAL PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updater NUMERIC NOT NULL,
    target NUMERIC NOT NULL,
    victory_fourths NUMERIC NOT NULL,
    guild_id NUMERIC NOT NULL
);

CREATE TABLE naval_victory_counts (
    user_id NUMERIC NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated TIMESTAMP WITH TIME ZONE NOT NULL,
    victory_fourths NUMERIC NOT NULL,
    guild_id NUMERIC NOT NULL,
    id BIGSERIAL PRIMARY KEY
);

CREATE TABLE naval_tackle_assist_count_changes (
    id BIGSERIAL PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updater NUMERIC NOT NULL,
    target NUMERIC NOT NULL,
    tackle_assists NUMERIC NOT NULL,
    guild_id NUMERIC NOT NULL
);

CREATE TABLE naval_tackle_assist_counts (
    user_id NUMERIC NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated TIMESTAMP WITH TIME ZONE NOT NULL,
    tackle_assists NUMERIC NOT NULL,
    guild_id NUMERIC NOT NULL,
    id BIGSERIAL PRIMARY KEY
);

CREATE UNIQUE INDEX unique_legion_kill_count_per_guild_per_user ON legion_kill_counts (user_id, guild_id);
CREATE UNIQUE INDEX unique_naval_victory_count_per_guild_per_user ON naval_victory_counts (user_id, guild_id);
CREATE UNIQUE INDEX unique_naval_tackle_assist_count_per_guild_per_user ON naval_tackle_assist_counts (user_id, guild_id);


INSERT INTO legion_kill_count_changes (
    created,
    updater,
    target,
    kills,
    guild_id
)
SELECT
    created,
    updater,
    target,
    total as kills,
    guild_id
FROM tracker_count_changes
WHERE
    stat = 'ground_kill';

INSERT INTO legion_kill_counts (
    user_id,
    created,
    updated,
    kills,
    guild_id
)
SELECT
    user_id,
    created,
    updated,
    total as kills,
    guild_id
FROM tracker_counts
WHERE
    stat = 'ground_kill';

INSERT INTO naval_victory_count_changes (
    created,
    updater,
    target,
    victory_fourths,
    guild_id
)
SELECT
    created,
    updater,
    target,
    total as victory_fourths,
    guild_id
FROM tracker_count_changes
WHERE
    stat = 'naval_victory';

INSERT INTO naval_victory_counts (
    user_id,
    created,
    updated,
    victory_fourths,
    guild_id
)
SELECT
    user_id,
    created,
    updated,
    total as victory_fourths,
    guild_id
FROM tracker_counts
WHERE
    stat = 'naval_victory';

INSERT INTO naval_tackle_assist_count_changes (
    created,
    updater,
    target,
    tackle_assists,
    guild_id
)
SELECT
    created,
    updater,
    target,
    total as tackle_assists,
    guild_id
FROM tracker_count_changes
WHERE
    stat = 'naval_tackle_assist';

INSERT INTO naval_tackle_assist_counts (
    user_id,
    created,
    updated,
    tackle_assists,
    guild_id
)
SELECT
    user_id,
    created,
    updated,
    total as tackle_assists,
    guild_id
FROM tracker_counts
WHERE
    stat = 'naval_tackle_assist';

DELETE FROM tracker_count_changes
WHERE stat IN ('naval_tackle_assist', 'naval_victory', 'ground_kill');

DELETE FROM tracker_counts
WHERE stat IN ('naval_tackle_assist', 'naval_victory', 'ground_kill');
