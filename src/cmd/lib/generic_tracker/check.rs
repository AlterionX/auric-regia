use bigdecimal::BigDecimal;
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
                        trc::error!("Bad value for `user` in `{} check` {:?}", stat.cmd_name(), opt);
                        return Err(RequestError::Internal(format!("Bad value for `user` in `{} check`.", stat.cmd_name()).into()));
                    };
                    user_id = u.id;
                }
                _ => {
                    trc::error!("Unknown option `{}` for `{} check`", stat.cmd_name(), opt.name);
                    return Err(RequestError::Internal(format!("Unknown option in `{} check`", stat.cmd_name()).into()));
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
            format!("We have {} saved personnel recorded for {}.", total.unwrap_or_default(), user_id.inner().mention())
        },
    }
}
