CREATE TABLE naval_tackle_assist_count_changes (
    id BIGSERIAL PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updater NUMERIC NOT NULL,
    target NUMERIC NOT NULL,
    tackle_assists NUMERIC NOT NULL,
    user_note VARCHAR(10000)
);

CREATE TABLE naval_tackle_assist_counts (
    id NUMERIC PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated TIMESTAMP WITH TIME ZONE NOT NULL,
    tackle_assists NUMERIC NOT NULL
);
