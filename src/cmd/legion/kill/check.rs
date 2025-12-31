use bigdecimal::ToPrimitive;
use serenity::all::{CommandInteraction, Mentionable, ResolvedOption, ResolvedValue, UserId};
use tracing as trc;

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db};

#[derive(Debug)]
pub struct Request(UserId);

impl Request {
    pub fn parse(cmd: &CommandInteraction, options: &[ResolvedOption]) -> Result<Self, RequestError> {
        let mut user_id = cmd.user.id;
        for opt in options {
            match opt.name {
                "user" => {
                    let ResolvedValue::User(u, _) = opt.value else {
                        trc::error!("Bad value for `user` in `legion kill check` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `user` in `legion kill check`.".into()));
                    };
                    user_id = u.id;
                }
                _ => {
                    trc::error!("Unknown option `{}` for `legion kill check`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `legion kill check`".into()));
                }
            }
        }

        Ok(Self(user_id))
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let guild_id = ctx.cmd.guild_id.ok_or_else(|| RequestError::User("Command must be run from within a guild.".into()))?;
        let display_kills = db::LegionKillCount::load_for(&ctx.db_cfg, self.0, guild_id).await.map(|n| n.kills.to_i64().unwrap_or(0)).unwrap_or(0);
        ctx.reply_restricted(format!("We have {display_kills} kills recorded for {}.", self.0.mention())).await
    }
}
