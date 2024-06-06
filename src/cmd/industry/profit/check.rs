use serenity::all::{CommandInteraction, Mentionable, ResolvedOption, ResolvedValue, UserId};
use tracing as trc;

use crate::{cmd::RequestError, db, discord::ExecutionContext};

#[derive(Debug)]
pub struct Request(UserId);

impl Request {
    pub fn parse(cmd: &CommandInteraction, options: &[ResolvedOption]) -> Result<Self, RequestError> {
        let mut user_id = cmd.user.id;
        for opt in options {
            match opt.name {
                "user" => {
                    let ResolvedValue::User(u, _) = opt.value else {
                        trc::error!("Bad value for `user` in `industry profits check` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `user` in `industry profits check`.".into()));
                    };
                    user_id = u.id;
                }
                _ => {
                    trc::error!("Unknown option `{}` for `industry profits check`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `industry profits check`".into()));
                }
            }
        }

        Ok(Self(user_id))
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let profits = db::IndustryProfitCount::load_for(&ctx.db_cfg, self.0).map(|record| record.alpha_united_earth_credits).unwrap_or(0.into());
        ctx.reply_restricted(format!("We have {profits} aUEC recorded for {}.", self.0.mention())).await
    }
}
