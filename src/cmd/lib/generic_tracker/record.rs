use bigdecimal::BigDecimal;
use tracing as trc;

use serenity::all::{CommandInteraction, Mentionable, ResolvedOption, ResolvedValue};

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db::{self, DiscordGuildId, DiscordUserId, TrackerStat}};

#[derive(Debug)]
pub struct Request {
    stat: TrackerStat,
    total: BigDecimal,
    user_id: DiscordUserId,
    guild_id: DiscordGuildId,
}

impl Request {
    pub fn parse(cmd: &CommandInteraction, stat: TrackerStat, options: &[ResolvedOption]) -> Result<Self, RequestError> {
        let guild_id = cmd.guild_id.ok_or_else(|| RequestError::User("Command must be run from within a guild.".into()))?.into();

        let mut total = 1;
        let mut user_id = cmd.user.id;
        for opt in options {
            match opt.name {
                "total" => {
                    let ResolvedValue::Integer(k) = opt.value else {
                        trc::error!("Bad value for `total` in `{} delete` {:?}", stat.cmd_name(), opt);
                        return Err(RequestError::Internal(format!("Bad value for `total` in `{} delete`.", stat.cmd_name()).into()));
                    };
                    if k < 0 {
                        trc::error!("Bad value for `total` in `{} delete` {:?}", stat.cmd_name(), opt);
                        return Err(RequestError::User(format!("Negative value for `total` in `{} delete`. Were you looking for `{} record`?", stat.cmd_name(), stat.cmd_name()).into()));
                    }
                    total = k;
                }
                "user" => {
                    let ResolvedValue::User(u, _) = opt.value else {
                        trc::error!("Bad value for `user` in `{} delete` {:?}", stat.cmd_name(), opt);
                        return Err(RequestError::Internal(format!("Bad value for `user` in `{} delete`.", stat.cmd_name()).into()));
                    };
                    user_id = u.id;
                }
                _ => {
                    trc::error!("Unknown option `{}` for `{} delete`", stat.cmd_name(), opt.name);
                    return Err(RequestError::Internal(format!("Unknown option in `{} delete`", stat.cmd_name()).into()));
                }
            }
        }
        let user_id = user_id.into();

        Ok(Self {
            stat,
            total: BigDecimal::from(total),
            user_id,
            guild_id,
        })
    }

    pub async fn execute(Self { stat, total, user_id, guild_id }: Self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let change = db::NewTrackerCountChange {
            stat,
            guild_id,
            updater: ctx.cmd.user.id.into(),
            target: user_id,
            total: total.clone(),
        };

        let Ok(new_total) = db::TrackerCount::adjust_count(&ctx.db_cfg, change).await else {
            trc::error!("Failed to update count for {} delete.", stat.cmd_name());
            return Err(RequestError::Internal("Count update failed".into()));
        };

        ctx.reply(format_delete_for_stat(stat, user_id, total, new_total)).await
    }
}

fn format_delete_for_stat(stat: TrackerStat, user_id: DiscordUserId, delta: BigDecimal, new_total: BigDecimal) -> String {
    match stat {
        TrackerStat::PersonnelSaved => {
            format!("Removed {} saved personnel from {} (total {}).", delta, user_id.inner().mention(), new_total)
        },
    }
}
