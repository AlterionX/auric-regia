// @generated automatically by Diesel CLI.

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
    tracker_count_changes (id) {
        id -> Int8,
        created -> Timestamptz,
        #[max_length = 500]
        stat -> Varchar,
        guild_id -> Numeric,
        updater -> Numeric,
        target -> Numeric,
        total -> Numeric,
        #[max_length = 10000]
        user_note -> Nullable<Varchar>,
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
    monthly_goals,
    tracker_count_changes,
    tracker_counts,
);
