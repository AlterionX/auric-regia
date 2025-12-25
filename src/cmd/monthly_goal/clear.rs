use serenity::all::{CommandInteraction, ResolvedOption};

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db};

#[derive(Debug)]
pub struct Request;

impl Request {
    pub fn parse(_cmd: &CommandInteraction, _options: &'_ [ResolvedOption]) -> Result<Self, RequestError> {
        Ok(Self)
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let Ok(_) = db::MonthlyGoal::clear_active(&ctx.db_cfg).await else {
            return Err(RequestError::Internal("Failure to write".into()));
        };

        ctx.reply_restricted("Monthly goals cleared.".into()).await?;

        Ok(())
    }
}

