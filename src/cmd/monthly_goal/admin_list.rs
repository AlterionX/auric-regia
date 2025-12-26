use std::borrow::Cow;

use serenity::all::{CommandInteraction, ResolvedOption, ResolvedValue};
use tracing as trc;

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db};

#[derive(Debug)]
pub struct Request<'a> {
    branch: Option<&'a str>
}

impl <'a> Request<'a> {
    pub fn parse(_cmd: &'a CommandInteraction, options: &'_ [ResolvedOption<'a>]) -> Result<Self, RequestError> {
        let mut branch = None;
        for opt in options {
            match opt.name {
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
            branch,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let goals = if let Some(branch) = self.branch {
            db::MonthlyGoal::load_active_for_branch(&ctx.db_cfg, branch).await
        } else {
            db::MonthlyGoal::load_all_active(&ctx.db_cfg).await
        };

        let Ok(goals) = goals else {
            return Err(RequestError::Internal("Failed to load monthly goals.".into()));
        };

        let msg: String = std::iter::once(Cow::Borrowed("- "))
            .chain(goals.iter()
                .map(|goal| Cow::Owned(format!("`{}` {}\n> {}", goal.shortname, goal.progress, goal.header)))
                .intersperse(Cow::Borrowed("\n- ")))
            .collect();
        ctx.reply_restricted(msg).await?;

        Ok(())
    }
}
