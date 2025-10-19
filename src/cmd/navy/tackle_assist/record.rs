use tracing as trc;
use bigdecimal::ToPrimitive;

use serenity::all::{CommandInteraction, Mention, ResolvedOption, ResolvedValue, UserId};

use crate::{cmd::RequestError, db, discord::ExecutionContext};

#[derive(Debug)]
pub struct Request {
    tackle_assists: i64,
    user_id: UserId,
}

impl Request {
    pub fn parse(cmd: &CommandInteraction, options: &[ResolvedOption]) -> Result<Self, RequestError> {
        let mut tackle_assists = 1;
        let mut user_id = cmd.user.id;
        for opt in options {
            match opt.name {
                "tackle_assists" => {
                    let ResolvedValue::Integer(k) = opt.value else {
                        trc::error!("Bad value for `tackle_assists` in `navy tackle_assist record` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `tackle_assists` in `navy tackle_assist record`.".into()));
                    };
                    if k < 0 {
                        trc::error!("Bad value for `tackle_assists` in `navy tackle_assist record` {:?}", opt);
                        return Err(RequestError::User("Negative value for `tackle_assists` in `navy tackle_assist record`. Were you looking for `navy tackle_assist delete`?".into()));
                    }
                    tackle_assists = k;
                }
                "user" => {
                    let ResolvedValue::User(u, _) = opt.value else {
                        trc::error!("Bad value for `user` in `navy tackle_assist record` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `user` in `navy tackle_assist record`.".into()));
                    };
                    user_id = u.id;
                }
                _ => {
                    trc::error!("Unknown option `{}` for `navy tackle_assist record`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `navy tackle_assist record`".into()));
                }
            }
        }
        Ok(Self {
            tackle_assists,
            user_id,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let tackle_assists_to_add = self.tackle_assists;
        let change = db::NewNavalTackleAssistCountChange {
            updater: u64::from(ctx.cmd.user.id).into(),
            target: u64::from(self.user_id).into(),
            tackle_assists: self.tackle_assists.into(),
        };

        let final_tackle_assists = match db::NavalTackleAssistCount::adjust_count(&ctx.db_cfg, change) {
            Ok(v) => v.to_i64().unwrap_or(0) as f64 / 4.,
            Err(e) => {
                trc::error!("Failed to update count for navy tackle assist record. err={e:?}");
                let _ = ctx.reply(format!("Something broke... please contact a mod")).await;
                return Ok(());
            }
        };

        ctx.reply(format!("Added {tackle_assists_to_add} tackle assists to {} (total {final_tackle_assists}).", Mention::User(self.user_id))).await
    }
}
