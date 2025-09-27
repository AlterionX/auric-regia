use bigdecimal::ToPrimitive;
use tracing as trc;

use serenity::all::{CommandInteraction, Error as SerenityError, ErrorResponse, GuildId, HttpError, Mention, ResolvedOption, ResolvedTarget, StatusCode, UserId};

use crate::{cmd::RequestError, discord::ExecutionContext};

#[derive(Debug)]
pub struct Request(GuildId);

const QUERY_LIMIT: i64 = 100;

impl Request {
    pub fn parse(cmd: &CommandInteraction, _options: &[ResolvedOption]) -> Result<Self, RequestError> {
        let Some(guild_id) = cmd.guild_id else {
            return Err(RequestError::User("This command must be used in a server.".into()).into());
        };

        Ok(Self(guild_id))
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        Ok(())
    }
}

