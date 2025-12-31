use serenity::all::{CommandInteraction, ResolvedOption, ResolvedValue};
use tracing as trc;

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db};

#[derive(Debug)]
pub struct Request<'a> {
    shortname: &'a str,
    branch: &'a str,
    header: Option<&'a str>,
    body: Option<&'a str>,
    progress: Option<i16>,
}

impl <'a> Request<'a> {
    pub fn parse(_cmd: &CommandInteraction, options: &'_ [ResolvedOption<'a>]) -> Result<Self, RequestError> {
        let mut shortname = None;
        let mut branch = "main";
        let mut header = None;
        let mut body = None;
        let mut progress = None;
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
                    branch = u;
                }
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
                        return Err(RequestError::Internal("Bad value for `body` in `monthly_goal set`.".into()));
                    };
                    body = Some(u);
                }
                "progress" => {
                    let ResolvedValue::Integer(u) = opt.value else {
                        trc::error!("Bad value for `progress` in `monthly_goal set` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `progress` in `monthly_goal set`.".into()));
                    };
                    let Ok(u) = i16::try_from(u) else {
                        trc::error!("Out of range (1 - 100) value for `progress` in `monthly_goal set` {:?}", opt);
                        return Err(RequestError::Internal("Out of range (1 - 100) value for `progress` in `monthly_goal set` {:?}".into()));
                    };
                    if !(0..=100).contains(&u) {
                        trc::error!("Out of range (0 - 100) value for `progress` in `monthly_goal set` {:?}", opt);
                        return Err(RequestError::Internal("Out of range (1 - 100) value for `progress` in `monthly_goal set` {:?}".into()));
                    };
                    progress = Some(u);
                }
                _ => {
                    trc::error!("Unknown option `{}` for `monthly_goal set`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `monthly_goal set`".into()));
                }
            }
        }

        let Some(shortname) = shortname else {
            trc::error!("Missing value for `shortname` in `monthly_goal set`");
            return Err(RequestError::Internal("Missing value for `shortname` in `monthly goal check`.".into()));
        };

        Ok(Self {
            shortname,
            branch,
            header,
            body,
            progress,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let guild_id = ctx.cmd.guild_id.ok_or_else(|| RequestError::User("Command must be run from within a guild.".into()))?;
        match db::MonthlyGoal::upsert(&ctx.db_cfg, db::NewMonthlyGoal {
            updater: u64::from(ctx.cmd.user.id).into(),
            shortname: self.shortname,
            tag: self.branch,
            header: self.header,
            body: self.body,
            progress: self.progress,
            guild_id: u64::from(guild_id).into(),
        }).await {
            Ok(_) => {},
            Err(e) => {
                return Err(RequestError::Internal(format!("Failure to write {:?}", e).into()));
            },
        };

        ctx.reply_restricted(format!("Updated monthly goal for {}", self.branch)).await?;

        Ok(())
    }
}
