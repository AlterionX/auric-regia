use tracing as trc;

use serenity::all::{CommandInteraction, Mention, ResolvedOption, ResolvedTarget, ResolvedValue, UserId};

use crate::{cmd::RequestError, db, discord::ExecutionContext};

#[derive(Debug)]
pub struct Request {
    victory_fourths: u64,
    user_id: UserId,
}

impl Request {
    pub fn parse(cmd: &CommandInteraction, options: &[ResolvedOption]) -> Result<Self, RequestError> {
        match cmd.data.name.as_str() {
            "Record One Naval Victory" => {
                let Some(ResolvedTarget::User(user, _member)) = cmd.data.target() else {
                    trc::error!("User context menu option does not have user target.");
                    return Err(RequestError::Internal("Missing user attached to context menu.".into()));
                };
                Ok(Self {
                    victory_fourths: 4,
                    user_id: user.id,
                })
            },
            _ => {
                let mut victories = 1.;
                let mut user_id = cmd.user.id;
                for opt in options {
                    match opt.name {
                        "victories" => {
                            let ResolvedValue::Number(k) = opt.value else {
                                trc::error!("Bad value for `victories` in `navy victory record` {:?}", opt);
                                return Err(RequestError::Internal("Bad value for `victories` in `navy victory record`.".into()));
                            };
                            if k < 0. {
                                trc::error!("Bad value for `victories` in `navy victory record` {:?}", opt);
                                return Err(RequestError::User("Negative value for `victories` in `navy victory record`. Were you looking for `navy victory delete`?".into()));
                            }
                            victories = k;
                        }
                        "user" => {
                            let ResolvedValue::User(u, _) = opt.value else {
                                trc::error!("Bad value for `user` in `navy victory record` {:?}", opt);
                                return Err(RequestError::Internal("Bad value for `user` in `navy victory record`.".into()));
                            };
                            user_id = u.id;
                        }
                        _ => {
                            trc::error!("Unknown option `{}` for `navy victory record`", opt.name);
                            return Err(RequestError::Internal("Unknown option in `navy victory record`".into()));
                        }
                    }
                }
                let integral = (victories * 4.).round() as u64;
                Ok(Self {
                    victory_fourths: integral,
                    user_id,
                })
            },
        }
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let victories_to_add = self.victory_fourths as f64 / 4.;

        let change = db::NewNavalVictoryCountChange {
            updater: u64::from(ctx.cmd.user.id).into(),
            target: u64::from(self.user_id).into(),
            victory_fourths: self.victory_fourths.into(),
        };

        let final_victories = match db::NavalVictoryCount::adjust_count(&ctx.db_cfg, change) {
            Ok(v) => v,
            Err(e) => {
                trc::error!("Failed to update count for navy victory record. err={e:?}");
                let _ = ctx.reply(format!("Something broke... please contact a mod")).await;
                return Ok(());
            }
        };

        ctx.reply(format!("Added {victories_to_add} victories to {} (total {final_victories}).", Mention::User(self.user_id))).await
    }
}
