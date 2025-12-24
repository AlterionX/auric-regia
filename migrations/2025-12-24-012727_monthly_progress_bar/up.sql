ALTER TABLE monthly_goals
    ADD COLUMN progress SMALLINT NOT NULL DEFAULT 0,
    ADD COLUMN shortname VARCHAR(50),
    ADD COLUMN active BOOLEAN NOT NULL DEFAULT FALSE;

UPDATE monthly_goals SET shortname = CAST(id AS VARCHAR(50));

ALTER TABLE monthly_goals ALTER COLUMN shortname SET NOT NULL;

CREATE UNIQUE INDEX monthly_goal_unique_active_shortname ON monthly_goals (shortname) WHERE active;
