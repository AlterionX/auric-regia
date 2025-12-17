use bigdecimal::ToPrimitive;
use tracing as trc;

use serenity::all::{CommandInteraction, DiscordJsonError, Error as SerenityError, ErrorResponse, GuildId, HttpError, ResolvedOption, StatusCode, UserId};

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db::NavalVictoryCount};

#[derive(Debug)]
pub struct Request {
    deleter: UserId,
    guild: GuildId,
}

const QUERY_LIMIT: i64 = 100;

impl Request {
    pub fn parse(cmd: &CommandInteraction, _options: &[ResolvedOption]) -> Result<Self, RequestError> {
        let Some(guild_id) = cmd.guild_id else {
            return Err(RequestError::User("This command must be used in a server.".into()));
        };

        Ok(Self {
            deleter: cmd.user.id,
            guild: guild_id,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let mut current_offset = 0;
        let mut records_to_delete = vec![];

        while let Some(active_records) = match NavalVictoryCount::load_asc(&ctx.db_cfg, current_offset, QUERY_LIMIT).await {
            Ok(v) => if v.is_empty() {
                None
            } else {
                current_offset += v.len() as i64;
                Some(v)
            },
            Err(e) => {
                trc::error!("Failed to get scoreboard items from top due to {e:?}.");
                return Err(RequestError::Internal("failed to get scoreboard items".into()));
            },
        } {
            for record in active_records {
                let member = match self.guild.member(&ctx.ctx, record.id.to_u64().unwrap()).await {
                    Ok(m) => Some(m),
                    Err(SerenityError::Http(HttpError::UnsuccessfulRequest(e @ ErrorResponse { status_code: StatusCode::NOT_FOUND, error: DiscordJsonError { code: 10007, .. }, .. }))) => {
                        trc::info!("navy victory member check: {e:?}");
                        None
                    },
                    Err(e) => {
                        trc::error!("Failed to look up guild member {:?} due to {e:?}.", record.id);
                        return Err(RequestError::Internal("failed to get scoreboard items".into()));
                    },
                };
                if member.is_none() {
                    trc::info!("Removing entry {:?}", record);
                    records_to_delete.push(record.id);
                }
            }
        }

        match NavalVictoryCount::delete(&ctx.db_cfg, self.deleter, records_to_delete.as_slice()).await {
            Ok(count) => if count == 0 {
                ctx.reply("No records deleted.".to_owned()).await
            } else if count == 1 {
                ctx.reply("Deleted records for 1 user.".to_owned()).await
            } else {
                ctx.reply(format!("Deleted records for {count} users.")).await
            },
            Err(e) => {
                trc::error!("Failed to delete {:?} guild members due to {e:?}.", records_to_delete);
                Err(RequestError::Internal("failed to clear scoreboard".into()))
            },
        }
    }
}
