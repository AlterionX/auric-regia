CREATE TABLE event_participation_count_changes (
    id BIGSERIAL PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updater NUMERIC NOT NULL,
    target NUMERIC NOT NULL,
    event_participation NUMERIC NOT NULL,
    user_note VARCHAR(10000)
);

CREATE TABLE event_participation_counts (
    id NUMERIC PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated TIMESTAMP WITH TIME ZONE NOT NULL,
    event_participation NUMERIC NOT NULL
);
