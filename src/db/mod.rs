mod navy;
mod legion;
mod industry;
mod event;
mod monthly_goal;
mod tracker;

pub use navy::*;
pub use legion::*;
pub use industry::*;
pub use event::*;
pub use monthly_goal::*;
pub use tracker::*;

mod discord_id_wrapping {
    pub use serenity::model::id::UserId as InternalDiscordUserId;
    pub use serenity::model::id::MessageId as InternalDiscordMessageId;
    pub use serenity::model::id::InteractionId as InternalDiscordInteractionId;
    pub use serenity::model::id::ChannelId as InternalDiscordChannelId;
    pub use serenity::model::id::GuildId as InternalDiscordGuildId;
    pub use serenity::model::id::RoleId as InternalDiscordRoleId;

    use diesel::{
        pg::Pg,
        sql_types::Numeric,
    };

    use diesel_pg_type_utils::{
        wrap::wrap_type,
        PgU64,
    };

    type DB = Pg;

    wrap_type! {
        #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
        DiscordUserId<DB>(Numeric > PgU64 > InternalDiscordUserId)
            |pgu| {
                (*pgu.inner()).into()
            }
            |u| {
                &PgU64::from(u64::from(u))
            }
    }
    impl From<InternalDiscordUserId> for DiscordUserId {
        fn from(user_id: InternalDiscordUserId) -> Self {
            Self(user_id)
        }
    }

    wrap_type! {
        #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
        DiscordMessageId<DB>(Numeric > PgU64 > InternalDiscordMessageId)
            |pgu| {
                (*pgu.inner()).into()
            }
            |u| {
                &PgU64::from(u64::from(u))
            }
    }
    impl From<InternalDiscordMessageId> for DiscordMessageId {
        fn from(message_id: InternalDiscordMessageId) -> Self {
            Self(message_id)
        }
    }

    wrap_type! {
        #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
        DiscordInteractionId<DB>(Numeric > PgU64 > InternalDiscordInteractionId)
            |pgu| {
                (*pgu.inner()).into()
            }
            |u| {
                &PgU64::from(u64::from(u))
            }
    }
    impl From<InternalDiscordInteractionId> for DiscordInteractionId {
        fn from(interaction_id: InternalDiscordInteractionId) -> Self {
            Self(interaction_id)
        }
    }

    wrap_type! {
        #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
        DiscordChannelId<DB>(Numeric > PgU64 > InternalDiscordChannelId)
            |pgu| {
                (*pgu.inner()).into()
            }
            |u| {
                &PgU64::from(u64::from(u))
            }
    }
    impl From<InternalDiscordChannelId> for DiscordChannelId {
        fn from(channel_id: InternalDiscordChannelId) -> Self {
            Self(channel_id)
        }
    }

    wrap_type! {
        #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
        DiscordGuildId<DB>(Numeric > PgU64 > InternalDiscordGuildId)
            |pgu| {
                (*pgu.inner()).into()
            }
            |u| {
                &PgU64::from(u64::from(u))
            }
    }
    impl From<InternalDiscordGuildId> for DiscordGuildId {
        fn from(guild_id: InternalDiscordGuildId) -> Self {
            Self(guild_id)
        }
    }

    wrap_type! {
        #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
        DiscordRoleId<DB>(Numeric > PgU64 > InternalDiscordRoleId)
            |pgu| {
                (*pgu.inner()).into()
            }
            |u| {
                &PgU64::from(u64::from(u))
            }
    }
    impl From<InternalDiscordRoleId> for DiscordRoleId {
        fn from(guild_id: InternalDiscordRoleId) -> Self {
            Self(guild_id)
        }
    }
}
pub use discord_id_wrapping::*;
