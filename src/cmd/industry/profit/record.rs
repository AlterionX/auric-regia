use tracing as trc;

use serenity::all::{CommandInteraction, Mention, ResolvedOption, ResolvedValue, UserId};

use crate::{cmd::RequestError, db, discord::ExecutionContext};

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
                        trc::error!("Bad value for `auec` in `industry profit record` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `auec` in `industry profit record`.".into()));
                    };
                    if k < 0 {
                        trc::error!("Bad value for `auec` in `industry profit record` {:?}", opt);
                        return Err(RequestError::User("Negative value for `auec` in `industry profit record`. Were you looking for `industry profit delete`?".into()));
                    }
                    auec = k as u64;
                }
                "user" => {
                    let ResolvedValue::User(u, _) = opt.value else {
                        trc::error!("Bad value for `user` in `industry profit record` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `user` in `industry profit record`.".into()));
                    };
                    user_id = u.id;
                }
                _ => {
                    trc::error!("Unknown option `{}` for `industry profit record`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `industry profit record`".into()));
                }
            }
        }
        Ok(Self {
            auec,
            user_id,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let change = db::NewIndustryProfitCountChange {
            updater: u64::from(ctx.cmd.user.id).into(),
            target: u64::from(self.user_id).into(),
            alpha_united_earth_credits: self.auec.into(),
        };

        let final_auec = match db::IndustryProfitCount::adjust_count(&ctx.db_cfg, change) {
            Ok(v) => v,
            Err(e) => {
                trc::error!("Failed to update count for industry profit record. err={e:?}");
                let _ = ctx.reply(format!("Something broke... please contact a mod")).await;
                return Ok(());
            }
        };

        ctx.reply(format!("Recorded {} aUEC in profit for {} (total {final_auec}).", self.auec, Mention::User(self.user_id))).await
    }
}
