use bigdecimal::{BigDecimal, ToPrimitive};
use serenity::all::{CommandInteraction, Mentionable, ResolvedOption, ResolvedValue};
use tracing as trc;

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db::{self, DiscordGuildId, DiscordUserId, TrackerStat}};

#[derive(Debug)]
pub struct Request {
    stat: TrackerStat,
    guild_id: DiscordGuildId,
    user_id: DiscordUserId,
}

impl Request {
    pub fn parse(cmd: &CommandInteraction, stat: TrackerStat, options: &[ResolvedOption]) -> Result<Self, RequestError> {
        let guild_id = cmd.guild_id.ok_or_else(|| RequestError::User("Command must be run from within a server.".into()))?.into();
        let mut user_id = cmd.user.id;
        for opt in options {
            match opt.name {
                "user" => {
                    let ResolvedValue::User(u, _) = opt.value else {
                        trc::error!("Bad value for `user` in `navy victory check` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `user` in `navy victory check`.".into()));
                    };
                    user_id = u.id;
                }
                _ => {
                    trc::error!("Unknown option `{}` for `navy victory check`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `navy victory check`".into()));
                }
            }
        }
        let user_id = user_id.into();

        Ok(Self {
            stat,
            guild_id,
            user_id,
        })
    }

    pub async fn execute(Self { stat, guild_id, user_id }: Self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let record = db::TrackerCount::load_for(&ctx.db_cfg, stat, guild_id, user_id).await;
        ctx.reply_restricted(format_stat_for_check(stat, user_id, record.map(|r| r.total))).await
    }
}

fn format_stat_for_check(stat: TrackerStat, user_id: DiscordUserId, total: Option<BigDecimal>) -> String {
    match stat {
        TrackerStat::PersonnelSaved => {
            let value = total.and_then(|t| t.to_i64()).unwrap_or(0);
            format!("We have {} victories recorded for {}.", value, user_id.inner().mention())
        },
    }
}
