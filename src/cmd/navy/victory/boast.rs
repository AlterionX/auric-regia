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
        let display_victories = db::NavalVictoryCount::load_for(&ctx.db_cfg, self.0).await.map(|n| n.victory_fourths.to_i64().unwrap_or(0) as f64 / 4.).unwrap_or(0.);
        ctx.reply_restricted(format!("@here {} has earned {display_victories} victories!", self.0.mention())).await
    }
}
