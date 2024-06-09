use serenity::all::{CommandInteraction, Mentionable, ResolvedOption, UserId};
// use tracing as trc;

use crate::{cmd::RequestError, db, discord::ExecutionContext};

#[derive(Debug)]
pub struct Request(UserId);

impl Request {
    pub fn parse(cmd: &CommandInteraction, _options: &[ResolvedOption]) -> Result<Self, RequestError> {
        Ok(Self(cmd.user.id))
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let count = db::EventParticipationCount::load_for(&ctx.db_cfg, self.0).map(|record| record.event_participation).unwrap_or(0.into());
        ctx.reply_restricted(format!("We have {count} events recorded for {}.", self.0.mention())).await
    }
}

