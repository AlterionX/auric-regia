CREATE TABLE naval_victory_count_changes (
    id BIGSERIAL PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updater NUMERIC NOT NULL,
    target NUMERIC NOT NULL,
    victory_fourths NUMERIC NOT NULL
);

CREATE TABLE naval_victory_counts (
    id NUMERIC PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated TIMESTAMP WITH TIME ZONE NOT NULL,
    victory_fourths NUMERIC NOT NULL
);
