use bigdecimal::BigDecimal;
use serenity::all::{CommandInteraction, Mentionable, ResolvedOption};
// use tracing as trc;

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db::{self, DiscordGuildId, DiscordUserId, TrackerStat}};

#[derive(Debug)]
pub struct Request {
    stat: TrackerStat,
    guild_id: DiscordGuildId,
    user_id: DiscordUserId,
}

impl Request {
    pub fn parse(cmd: &CommandInteraction, stat: TrackerStat, _options: &[ResolvedOption]) -> Result<Self, RequestError> {
        let guild_id = cmd.guild_id.ok_or_else(|| RequestError::User("Command must be run from within a server.".into()))?.into();
        let user_id = cmd.user.id.into();

        Ok(Self {
            stat,
            guild_id,
            user_id,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let Self { stat, guild_id, user_id } = self;
        let record = db::TrackerCount::load_for(&ctx.db_cfg, stat, guild_id, user_id).await;
        ctx.reply_restricted(format_stat_for_boast(stat, user_id, record.map(|r| r.total))).await
    }
}

fn format_stat_for_boast(stat: TrackerStat, user_id: DiscordUserId, total: Option<BigDecimal>) -> String {
    match stat {
        TrackerStat::PersonnelSaved => {
            format!("@here {} has saved {} personnel!", user_id.inner().mention(), total.unwrap_or_default())
        },
    }
}
