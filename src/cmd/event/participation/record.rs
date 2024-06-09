use tracing as trc;

use serenity::all::{CommandInteraction, Mention, ResolvedOption, ResolvedValue, UserId};

use crate::{cmd::RequestError, db, discord::ExecutionContext};

#[derive(Debug)]
pub struct Request<'a> {
    event_participation: u64,
    user_id: UserId,
    note: Option<&'a str>,
}

impl <'a> Request<'a> {
    pub fn parse(cmd: &CommandInteraction, options: &'_ [ResolvedOption<'a>]) -> Result<Self, RequestError> {
        let mut event_participation = 1;
        let mut user_id = cmd.user.id;
        let mut note = None;
        for opt in options {
            match opt.name {
                "count" => {
                    let ResolvedValue::Integer(k) = opt.value else {
                        trc::error!("Bad value for `count` in `event participation count` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `count` in `event participation count`.".into()));
                    };
                    if k < 0 {
                        trc::error!("Bad value for `count` in `event participation count` {:?}", opt);
                        return Err(RequestError::User("Negative value for `count` in `event participation record`. Were you looking for `event participation remove`?".into()));
                    }
                    event_participation = k.try_into().unwrap();
                }
                "user" => {
                    let ResolvedValue::User(u, _) = opt.value else {
                        trc::error!("Bad value for `user` in `event participation record` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `user` in `event participation record`.".into()));
                    };
                    user_id = u.id;
                }
                "note" => {
                    let ResolvedValue::String(s) = opt.value else {
                        trc::error!("Bad value for `note` in `event participation record` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `note` in `event participation record`.".into()));
                    };
                    note = Some(s);
                }
                _ => {
                    trc::error!("Unknown option `{}` for `event participation record`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `event participation record`".into()));
                }
            }
        }
        Ok(Self {
            event_participation,
            user_id,
            note,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let change = db::NewEventParticipationCountChange {
            updater: u64::from(ctx.cmd.user.id).into(),
            target: u64::from(self.user_id).into(),
            event_participation: self.event_participation.into(),
            user_note: self.note.map(|s| s.to_owned()),
        };

        let final_count = match db::EventParticipationCount::adjust_count(&ctx.db_cfg, change) {
            Ok(v) => v,
            Err(e) => {
                trc::error!("Failed to update count for event participation record. err={e:?}");
                let _ = ctx.reply("Something broke... please contact a mod".to_owned()).await;
                return Ok(());
            }
        };

        ctx.reply(format!("Added {} events to {} (total {final_count}).", self.event_participation, Mention::User(self.user_id))).await
    }
}

