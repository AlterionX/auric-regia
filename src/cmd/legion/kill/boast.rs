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
        let display_kills = db::LegionKillCount::load_for(&ctx.db_cfg, self.0).await.map(|n| n.kills.to_i64().unwrap_or(0)).unwrap_or(0);
        ctx.reply_restricted(format!("@here {} has {display_kills} confirmed kills!", self.0.mention())).await
    }
}
