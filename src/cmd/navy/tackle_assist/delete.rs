use std::ops::Neg;
use bigdecimal::{BigDecimal, ToPrimitive};
use tracing as trc;

use serenity::all::{CommandInteraction, Mention, ResolvedOption, ResolvedValue, UserId};

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db};

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
                        trc::error!("Bad value for `tackle_assists` in `navy tackle_assist delete` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `tackle_assist` in `navy tackle_assist delete`.".into()));
                    };
                    if k < 0 {
                        trc::error!("Bad value for `tackle_assists` in `navy tackle_assist delete` {:?}", opt);
                        return Err(RequestError::User("Negative value for `tackle_assists` in `navy tackle_assist delete`. Were you looking for `navy tackle_assist record`?".into()));
                    }
                    tackle_assists = k;
                }
                "user" => {
                    let ResolvedValue::User(u, _) = opt.value else {
                        trc::error!("Bad value for `user` in `navy tackle_assist delete` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `user` in `navy tackle_assist delete`.".into()));
                    };
                    user_id = u.id;
                }
                _ => {
                    trc::error!("Unknown option `{}` for `navy tackle_assist delete`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `navy tackle_assist delete`".into()));
                }
            }
        }
        Ok(Self {
            tackle_assists,
            user_id,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let tackle_assists_to_remove = self.tackle_assists;

        let change = db::NewNavalTackleAssistCountChange {
            updater: u64::from(ctx.cmd.user.id).into(),
            target: u64::from(self.user_id).into(),
            tackle_assists: BigDecimal::from(self.tackle_assists).neg(),
        };

        let Ok(new_tackle_assists) = db::NavalTackleAssistCount::adjust_count(&ctx.db_cfg, change).await else {
            trc::error!("Failed to update count for navy tackle_assist delete.");
            let _ = ctx.reply(format!("Something broke... please contact a mod")).await;
            return Ok(());
        };
        let final_tackle_assists = new_tackle_assists.to_i64().unwrap_or(0);

        ctx.reply(format!("Removed {tackle_assists_to_remove} tackle assists from {} (total {final_tackle_assists}).", Mention::User(self.user_id))).await
    }
}
