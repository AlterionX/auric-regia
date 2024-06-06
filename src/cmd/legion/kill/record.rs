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
                        trc::error!("Bad value for `kills` in `legion kill record` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `kills` in `legion kill record`.".into()));
                    };
                    if k < 0 {
                        trc::error!("Bad value for `kills` in `legion kill record` {:?}", opt);
                        return Err(RequestError::User("Negative value for `kills` in `legion kill record`. Were you looking for `legion kill delete`?".into()));
                    }
                    kills = k;
                }
                "user" => {
                    let ResolvedValue::User(u, _) = opt.value else {
                        trc::error!("Bad value for `user` in `legion kill record` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `user` in `legion kill record`.".into()));
                    };
                    user_id = u.id;
                }
                _ => {
                    trc::error!("Unknown option `{}` for `legion kill record`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `legion kill record`".into()));
                }
            }
        }
        Ok(Self {
            kills,
            user_id,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let kills_to_add = self.kills;

        let change = db::NewLegionKillCountChange {
            updater: u64::from(ctx.cmd.user.id).into(),
            target: u64::from(self.user_id).into(),
            kills: self.kills.into(),
        };

        let final_kills = match db::LegionKillCount::adjust_count(&ctx.db_cfg, change) {
            Ok(v) => v,
            Err(e) => {
                trc::error!("Failed to update count for legion kill record. err={e:?}");
                return Err(RequestError::Internal("Failed to update count for legion kill record.".into()));
            }
        };

        ctx.reply(format!("Added {kills_to_add} kill(s) to {} (total {final_kills}).", Mention::User(self.user_id))).await
    }
}
