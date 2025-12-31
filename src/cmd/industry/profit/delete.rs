use std::ops::Neg;
use bigdecimal::BigDecimal;
use tracing as trc;

use serenity::all::{CommandInteraction, Mention, ResolvedOption, ResolvedValue, UserId};

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db};

#[derive(Debug)]
pub struct Request {
    auec: u64,
    user_id: UserId,
}

impl Request {
    pub fn parse(cmd: &CommandInteraction, options: &[ResolvedOption]) -> Result<Self, RequestError> {
        let mut auec = 1000;
        let mut user_id = cmd.user.id;
        for opt in options {
            match opt.name {
                "auec" => {
                    let ResolvedValue::Integer(k) = opt.value else {
                        trc::error!("Bad value for `auec` in `industry profit delete` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `auec` in `industry profit delete`.".into()));
                    };
                    if k < 0 {
                        trc::error!("Bad value for `auec` in `industry profit delete` {:?}", opt);
                        return Err(RequestError::User("Negative value for `auec` in `industry profit delete`. Were you looking for `industry profit record`?".into()));
                    }
                    auec = k as u64;
                }
                "user" => {
                    let ResolvedValue::User(u, _) = opt.value else {
                        trc::error!("Bad value for `user` in `industry profit delete` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `user` in `industry profit delete`.".into()));
                    };
                    user_id = u.id;
                }
                _ => {
                    trc::error!("Unknown option `{}` for `industry profit delete`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `industry profit delete`".into()));
                }
            }
        }
        Ok(Self {
            auec,
            user_id,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let guild_id = ctx.cmd.guild_id.ok_or_else(|| RequestError::User("Command must be run from within a guild.".into()))?;
        let change = db::NewIndustryProfitCountChange {
            updater: u64::from(ctx.cmd.user.id).into(),
            target: u64::from(self.user_id).into(),
            alpha_united_earth_credits: BigDecimal::from(self.auec).neg(),
            guild_id: u64::from(guild_id).into(),
        };

        let Ok(final_auec) = db::IndustryProfitCount::adjust_count(&ctx.db_cfg, change).await else {
            trc::error!("Failed to update count for industry profit delete.");
            let _ = ctx.reply(format!("Something broke... please contact a mod")).await;
            return Ok(());
        };

        ctx.reply(format!("Removed {} aUEC in profit from {} (total {final_auec}).", self.auec, Mention::User(self.user_id))).await
    }
}
