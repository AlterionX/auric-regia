use bigdecimal::{BigDecimal, FromPrimitive, Signed};
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

        let mut total = stat.default_add_remove_total();
        let mut user_id = cmd.user.id;
        for opt in options {
            match opt.name {
                "stat" => {},
                "total" => {
                    let k: BigDecimal = match opt.value {
                        ResolvedValue::Integer(k) => k.into(),
                        ResolvedValue::Number(k) => match BigDecimal::from_f64(k) {
                            Some(k) => k,
                            None => {
                                trc::error!("Bad value for `total` in `{} record` {:?}", stat.cmd_name(), opt);
                                return Err(RequestError::Internal(format!("Bad value for `total` in `{} record`.", stat.cmd_name()).into()));
                            },
                        },
                        _ => {
                            trc::error!("Bad value for `total` in `{} delete` {:?}", stat.cmd_name(), opt);
                            return Err(RequestError::Internal(format!("Bad value for `total` in `{} delete`.", stat.cmd_name()).into()));
                        },
                    };
                    if k.is_negative() {
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
                    trc::error!("Unknown option `{}` for `{} delete`", opt.name, stat.cmd_name());
                    return Err(RequestError::Internal("Unknown option in `{} delete`".into()));
                }
            }
        }
        let user_id = user_id.into();

        total *= stat.denominator();

        Ok(Self {
            stat,
            total,
            user_id,
            guild_id,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let Self { stat, total, user_id, guild_id } = self;
        let change = db::NewTrackerCountChange {
            stat,
            guild_id,
            updater: ctx.cmd.user.id.into(),
            target: user_id,
            total: -total.clone(),
            user_note: None,
        };

        let Ok(new_total) = db::TrackerCount::adjust_count(&ctx.db_cfg, change).await else {
            trc::error!("Failed to update count for {} delete.", stat.cmd_name());
            return Err(RequestError::Internal("Count update failed".into()));
        };

        ctx.reply(format_delete_for_stat(stat, user_id, total, new_total)).await
    }
}

fn format_delete_for_stat(stat: TrackerStat, user_id: DiscordUserId, delta: BigDecimal, new_total: BigDecimal) -> String {
    format!(
        "Removed {} from {} (total {}).",
        stat.format_count(delta),
        user_id.inner().mention(),
        stat.display_value(new_total),
    )
}
