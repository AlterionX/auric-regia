use serenity::all::{CommandInteraction, ResolvedOption, ResolvedValue};
use tracing as trc;

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db};

#[derive(Debug)]
pub struct Request<'a> {
    shortname: Option<&'a str>
}

impl <'a> Request<'a> {
    pub fn parse(_cmd: &'a CommandInteraction, options: &'_ [ResolvedOption<'a>]) -> Result<Self, RequestError> {
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
                _ => {
                    trc::error!("Unknown option `{}` for `monthly_goal set`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `monthly_goal set`".into()));
                }
            }
        }

        Ok(Self {
            shortname,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        ctx.reply_restricted(format!("")).await?;

        Ok(())
    }
}
