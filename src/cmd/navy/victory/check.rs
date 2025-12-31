use bigdecimal::ToPrimitive;
use serenity::all::{CommandInteraction, Mentionable, ResolvedOption, ResolvedTarget, ResolvedValue, UserId};
use tracing as trc;

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db};

#[derive(Debug)]
pub struct Request(UserId);

impl Request {
    pub fn parse(cmd: &CommandInteraction, options: &[ResolvedOption]) -> Result<Self, RequestError> {
        let uid = if cmd.data.name == "Check Naval Victories" {
            let Some(ResolvedTarget::User(user, _member)) = cmd.data.target() else {
                trc::error!("User context menu option does not have user target.");
                return Err(RequestError::Internal("Missing user attached to context menu.".into()));
            };
            user.id
        } else {
            let mut user_id = cmd.user.id;
            for opt in options {
                match opt.name {
                    "user" => {
                        let ResolvedValue::User(u, _) = opt.value else {
                            trc::error!("Bad value for `user` in `navy victory check` {:?}", opt);
                            return Err(RequestError::Internal("Bad value for `user` in `navy victory check`.".into()));
                        };
                        user_id = u.id;
                    }
                    _ => {
                        trc::error!("Unknown option `{}` for `navy victory check`", opt.name);
                        return Err(RequestError::Internal("Unknown option in `navy victory check`".into()));
                    }
                }
            }
            user_id
        };

        Ok(Self(uid))
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let guild_id = ctx.cmd.guild_id.ok_or_else(|| RequestError::User("Command must be run from within a guild.".into()))?;
        let display_victories = db::NavalVictoryCount::load_for(&ctx.db_cfg, self.0, guild_id).await.map(|n| n.victory_fourths.to_i64().unwrap_or(0) as f64 / 4.).unwrap_or(0.);
        ctx.reply_restricted(format!("We have {display_victories} victories recorded for {}.", self.0.mention())).await
    }
}
