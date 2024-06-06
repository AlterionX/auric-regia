CREATE TABLE industry_profit_count_changes (
    id BIGSERIAL PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updater NUMERIC NOT NULL,
    target NUMERIC NOT NULL,
    alpha_united_earth_credits NUMERIC NOT NULL
);

CREATE TABLE industry_profit_counts (
    id NUMERIC PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated TIMESTAMP WITH TIME ZONE NOT NULL,
    alpha_united_earth_credits NUMERIC NOT NULL
);
