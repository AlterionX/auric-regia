use bigdecimal::ToPrimitive;
use serenity::all::{CommandInteraction, Mentionable, ResolvedOption, UserId};
// use tracing as trc;

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db};

#[derive(Debug)]
pub struct Request(UserId);

impl Request {
    pub fn parse(cmd: &CommandInteraction, _options: &[ResolvedOption]) -> Result<Self, RequestError> {
        Ok(Self(cmd.user.id))
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let guild_id = ctx.cmd.guild_id.ok_or_else(|| RequestError::User("Command must be run from within a guild.".into()))?;
        let display_tackle_assists = db::NavalTackleAssistCount::load_for(&ctx.db_cfg, self.0, guild_id).await.map(|n| n.tackle_assists.to_i64().unwrap_or(0)).unwrap_or(0);
        ctx.reply_restricted(format!("@here {} has earned {display_tackle_assists} tackle assists!", self.0.mention())).await
    }
}
