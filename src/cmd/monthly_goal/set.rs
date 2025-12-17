use serenity::all::{CommandInteraction, ResolvedOption, ResolvedValue};
use tracing as trc;

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db};

#[derive(Debug)]
pub struct Request<'a> {
    branch: &'a str,
    header: &'a str,
    body: &'a str,
}

impl <'a> Request<'a> {
    pub fn parse(_cmd: &CommandInteraction, options: &'_ [ResolvedOption<'a>]) -> Result<Self, RequestError> {
        let mut header = None;
        let mut body = None;
        let mut branch = "main";
        for opt in options {
            match opt.name {
                "header" => {
                    let ResolvedValue::String(u) = opt.value else {
                        trc::error!("Bad value for `header` in `monthly_goal set` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `header` in `mongthly_goal set`.".into()));
                    };
                    header = Some(u);
                }
                "body" => {
                    let ResolvedValue::String(u) = opt.value else {
                        trc::error!("Bad value for `body` in `monthly_goal set` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `body` in `monthly_goal check`.".into()));
                    };
                    body = Some(u);
                }
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

        let Some(header) = header else {
            trc::error!("Missing value for `header` in `monthly_goal set`");
            return Err(RequestError::Internal("Missing value for `header` in `monthly goal check`.".into()));
        };
        let Some(body) = body else {
            trc::error!("Missing value for `body` in `monthly_goal set`");
            return Err(RequestError::Internal("Missing value for `body` in `monthly goal check`.".into()));
        };

        Ok(Self {
            branch,
            header,
            body
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let Ok(_) = db::MonthlyGoal::create(&ctx.db_cfg, db::NewMonthlyGoal {
            updater: u64::from(ctx.cmd.user.id).into(),
            tag: self.branch,
            header: self.header,
            body: self.body,
        }).await else {
            return Err(RequestError::Internal("Failure to write".into()));
        };

        ctx.reply_restricted(format!("Updated monthly goal for {}", self.branch)).await?;

        Ok(())
    }
}
