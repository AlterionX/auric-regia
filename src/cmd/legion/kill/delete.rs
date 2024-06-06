use std::ops::Neg;
use bigdecimal::BigDecimal;
use tracing as trc;

use serenity::all::{CommandInteraction, Mention, ResolvedOption, ResolvedValue, UserId};

use crate::{cmd::RequestError, db, discord::ExecutionContext};

#[derive(Debug)]
pub struct Request {
    kills: i64,
    user_id: UserId,
}

impl Request {
    pub fn parse(cmd: &CommandInteraction, options: &[ResolvedOption]) -> Result<Self, RequestError> {
        let mut kills = 1;
        let mut user_id = cmd.user.id;
        for opt in options {
            match opt.name {
                "kills" => {
                    let ResolvedValue::Integer(k) = opt.value else {
                        trc::error!("Bad value for `kills` in `legion kill delete` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `kills` in `legion kill delete`.".into()));
                    };
                    if k < 0 {
                        trc::error!("Bad value for `kills` in `legion kill delete` {:?}", opt);
                        return Err(RequestError::User("Negative value for `kills` in `legion kill delete`. Were you looking for `legion kill record`?".into()));
                    }
                    kills = k;
                }
                "user" => {
                    let ResolvedValue::User(u, _) = opt.value else {
                        trc::error!("Bad value for `user` in `legion kill delete` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `user` in `legion kill delete`.".into()));
                    };
                    user_id = u.id;
                }
                _ => {
                    trc::error!("Unknown option `{}` for `legion kill delete`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `legion kill delete`".into()));
                }
            }
        }
        Ok(Self {
            kills,
            user_id,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let kills_to_remove = self.kills;

        let change = db::NewLegionKillCountChange {
            updater: u64::from(ctx.cmd.user.id).into(),
            target: u64::from(self.user_id).into(),
            kills: BigDecimal::from(self.kills).neg(),
        };

        let final_kills = match db::LegionKillCount::adjust_count(&ctx.db_cfg, change) {
            Ok(v) => v,
            Err(e) => {
                trc::error!("Failed to update count for legion kill delete. err={e:?}");
                return Err(RequestError::Internal("Failed to update count for legion kill delete.".into()));
            }
        };

        ctx.reply(format!("Removed {kills_to_remove} kill(s) from {} (total {final_kills}).", Mention::User(self.user_id))).await
    }
}
