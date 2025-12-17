use serenity::all::{CommandInteraction, ResolvedOption, ResolvedValue};
use tracing as trc;

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db};

#[derive(Debug)]
pub struct Request<'a> {
    branch: &'a str,
}

impl<'a> Request<'a> {
    pub fn parse(_cmd: &'a CommandInteraction, options: &'_ [ResolvedOption<'a>]) -> Result<Self, RequestError> {
        let mut branch = "main";
        for opt in options {
            match opt.name {
                "branch" => {
                    let ResolvedValue::String(u) = opt.value else {
                        trc::error!("Bad value for `branch` in `monthly_goal set` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `branch` in `monthly_goal set`.".into()));
                    };
                    branch = u;
                }
                _ => {
                    trc::error!("Unknown option `{}` for `monthly_goal set`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `monthly_goal set`".into()));
                }
            }
        }

        Ok(Self {
            branch,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let Ok(opt_data) = db::MonthlyGoal::load_for(&ctx.db_cfg, self.branch).await else {
            return Err(RequestError::Internal("Failed to load monthly goal.".into()));
        };

        match opt_data {
            Some(data) => ctx.reply_restricted(format!("[{}] _*{}*_\n{}", data.tag, data.header, data.body)).await?,
            None => ctx.reply_restricted("No goal set!".into()).await?,
        }

        Ok(())
    }
}

