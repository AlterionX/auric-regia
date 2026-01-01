use tracing as trc;
use serenity::all::{CommandInteraction, ResolvedOption, ResolvedValue};

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db};

#[derive(Debug)]
pub struct Request<'a> {
    shortname: Option<&'a str>,
    branch: Option<&'a str>,
}

impl<'a> Request<'a> {
    pub fn parse(_cmd: &CommandInteraction, options: &'_ [ResolvedOption<'a>]) -> Result<Self, RequestError> {
        let mut branch = None;
        let mut shortname = None;
        for opt in options {
            match opt.name {
                "shortname" => {
                    let ResolvedValue::String(u) = opt.value else {
                        trc::error!("Bad value for `shortname` in `monthly_goal set` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `shortname` in `monthly_goal set`.".into()));
                    };
                    shortname = Some(u);
                }
                "branch" => {
                    let ResolvedValue::String(u) = opt.value else {
                        trc::error!("Bad value for `branch` in `monthly_goal set` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `branch` in `monthly_goal set`.".into()));
                    };
                    branch = Some(u);
                }
                _ => {
                    trc::error!("Unknown option `{}` for `monthly_goal set`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `monthly_goal set`".into()));
                }
            }
        }

        Ok(Self {
            shortname,
            branch,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let guild_id = ctx.cmd.guild_id.ok_or_else(|| RequestError::User("Command must be run from within a guild.".into()))?;
        if let Some(branch) = self.branch {
            let Ok(_) = db::MonthlyGoal::clear_active_by_tag(&ctx.db_cfg, guild_id, branch).await else {
                return Err(RequestError::Internal("Failure to write".into()));
            };
        }

        if let Some(shortname) = self.shortname {
            // TODO Detect if shortname doesn't actually exist
            let Ok(_) = db::MonthlyGoal::clear_active_by_shortname(&ctx.db_cfg, guild_id, shortname).await else {
                return Err(RequestError::Internal("Failure to write".into()));
            };
        }

        if self.shortname.is_none() && self.branch.is_none() {
            let Ok(_) = db::MonthlyGoal::clear_active(&ctx.db_cfg, guild_id).await else {
                return Err(RequestError::Internal("Failure to write".into()));
            };
        }

        ctx.reply_restricted("Monthly goals cleared.".into()).await?;

        Ok(())
    }
}

