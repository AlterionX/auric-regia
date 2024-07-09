use std::ops::Neg;
use bigdecimal::{BigDecimal, ToPrimitive};
use tracing as trc;

use serenity::all::{CommandInteraction, Mention, ResolvedOption, ResolvedValue, UserId};

use crate::{cmd::RequestError, db, discord::ExecutionContext};

#[derive(Debug)]
pub struct Request {
    victory_fourths: u64,
    user_id: UserId,
}

impl Request {
    pub fn parse(cmd: &CommandInteraction, options: &[ResolvedOption]) -> Result<Self, RequestError> {
        let mut victories = 1.;
        let mut user_id = cmd.user.id;
        for opt in options {
            match opt.name {
                "victories" => {
                    let ResolvedValue::Number(k) = opt.value else {
                        trc::error!("Bad value for `victories` in `navy victory delete` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `victories` in `navy victory delete`.".into()));
                    };
                    if k < 0. {
                        trc::error!("Bad value for `victories` in `navy victory delete` {:?}", opt);
                        return Err(RequestError::User("Negative value for `victories` in `navy victory delete`. Were you looking for `navy victory record`?".into()));
                    }
                    victories = k;
                }
                "user" => {
                    let ResolvedValue::User(u, _) = opt.value else {
                        trc::error!("Bad value for `user` in `navy victory delete` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `user` in `navy victory delete`.".into()));
                    };
                    user_id = u.id;
                }
                _ => {
                    trc::error!("Unknown option `{}` for `navy victory delete`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `navy victory delete`".into()));
                }
            }
        }
        let integral = (victories * 4.).round() as u64;
        Ok(Self {
            victory_fourths: integral,
            user_id,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let victories_to_remove = self.victory_fourths as f64 / 4.;

        let change = db::NewNavalVictoryCountChange {
            updater: u64::from(ctx.cmd.user.id).into(),
            target: u64::from(self.user_id).into(),
            victory_fourths: BigDecimal::from(self.victory_fourths).neg(),
        };

        let Ok(new_victory_fourths) = db::NavalVictoryCount::adjust_count(&ctx.db_cfg, change) else {
            trc::error!("Failed to update count for navy victory delete.");
            let _ = ctx.reply(format!("Something broke... please contact a mod")).await;
            return Ok(());
        };
        let final_victories = new_victory_fourths.to_i64().unwrap_or(0) as f64 / 4.;

        ctx.reply(format!("Removed {victories_to_remove} victories from {} (total {final_victories}).", Mention::User(self.user_id))).await
    }
}
