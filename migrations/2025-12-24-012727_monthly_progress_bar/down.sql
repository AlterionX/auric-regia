DROP INDEX monthly_goal_unique_active_shortname;
ALTER TABLE monthly_goals DROP COLUMN progress, DROP COLUMN shortname, DROP COLUMN disabled;
