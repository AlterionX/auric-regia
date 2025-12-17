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
        let profit = db::IndustryProfitCount::load_for(&ctx.db_cfg, self.0).await.map(|record| record.alpha_united_earth_credits).unwrap_or(0.into());
        ctx.reply_restricted(format!("@here {} has earned {profit} aUEC!", self.0.mention())).await
    }
}
