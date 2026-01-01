// @generated automatically by Diesel CLI.

diesel::table! {
    event_participation_count_changes (id) {
        id -> Int8,
        created -> Timestamptz,
        updater -> Numeric,
        target -> Numeric,
        event_participation -> Numeric,
        #[max_length = 10000]
        user_note -> Nullable<Varchar>,
        guild_id -> Numeric,
    }
}

diesel::table! {
    event_participation_counts (id) {
        user_id -> Numeric,
        created -> Timestamptz,
        updated -> Timestamptz,
        event_participation -> Numeric,
        guild_id -> Numeric,
        id -> Int8,
    }
}

diesel::table! {
    industry_profit_count_changes (id) {
        id -> Int8,
        created -> Timestamptz,
        updater -> Numeric,
        target -> Numeric,
        alpha_united_earth_credits -> Numeric,
        guild_id -> Numeric,
    }
}

diesel::table! {
    industry_profit_counts (id) {
        user_id -> Numeric,
        created -> Timestamptz,
        updated -> Timestamptz,
        alpha_united_earth_credits -> Numeric,
        guild_id -> Numeric,
        id -> Int8,
    }
}

diesel::table! {
    legion_kill_count_changes (id) {
        id -> Int8,
        created -> Timestamptz,
        updater -> Numeric,
        target -> Numeric,
        kills -> Numeric,
        guild_id -> Numeric,
    }
}

diesel::table! {
    legion_kill_counts (id) {
        user_id -> Numeric,
        created -> Timestamptz,
        updated -> Timestamptz,
        kills -> Numeric,
        guild_id -> Numeric,
        id -> Int8,
    }
}

diesel::table! {
    monthly_goals (id) {
        id -> Int8,
        created -> Timestamptz,
        updater -> Numeric,
        #[max_length = 100]
        tag -> Varchar,
        #[max_length = 256]
        header -> Varchar,
        #[max_length = 4096]
        body -> Varchar,
        progress -> Int2,
        #[max_length = 50]
        shortname -> Varchar,
        disabled -> Nullable<Timestamptz>,
        guild_id -> Numeric,
    }
}

diesel::table! {
    naval_tackle_assist_count_changes (id) {
        id -> Int8,
        created -> Timestamptz,
        updater -> Numeric,
        target -> Numeric,
        tackle_assists -> Numeric,
        #[max_length = 10000]
        user_note -> Nullable<Varchar>,
        guild_id -> Numeric,
    }
}

diesel::table! {
    naval_tackle_assist_counts (id) {
        user_id -> Numeric,
        created -> Timestamptz,
        updated -> Timestamptz,
        tackle_assists -> Numeric,
        guild_id -> Numeric,
        id -> Int8,
    }
}

diesel::table! {
    naval_victory_count_changes (id) {
        id -> Int8,
        created -> Timestamptz,
        updater -> Numeric,
        target -> Numeric,
        victory_fourths -> Numeric,
        guild_id -> Numeric,
    }
}

diesel::table! {
    naval_victory_counts (id) {
        user_id -> Numeric,
        created -> Timestamptz,
        updated -> Timestamptz,
        victory_fourths -> Numeric,
        guild_id -> Numeric,
        id -> Int8,
    }
}

diesel::table! {
    tracker_count_changes (id) {
        id -> Int8,
        created -> Timestamptz,
        #[max_length = 500]
        stat -> Varchar,
        guild_id -> Numeric,
        updater -> Numeric,
        target -> Numeric,
        total -> Numeric,
    }
}

diesel::table! {
    tracker_counts (id) {
        id -> Int8,
        created -> Timestamptz,
        updated -> Timestamptz,
        #[max_length = 100]
        stat -> Varchar,
        guild_id -> Numeric,
        user_id -> Numeric,
        total -> Numeric,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    event_participation_count_changes,
    event_participation_counts,
    industry_profit_count_changes,
    industry_profit_counts,
    legion_kill_count_changes,
    legion_kill_counts,
    monthly_goals,
    naval_tackle_assist_count_changes,
    naval_tackle_assist_counts,
    naval_victory_count_changes,
    naval_victory_counts,
    tracker_count_changes,
    tracker_counts,
);
