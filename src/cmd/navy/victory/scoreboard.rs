use std::marker::PhantomData;
use bigdecimal::ToPrimitive;
use tracing as trc;

use serenity::all::{CommandInteraction, Mention, ResolvedOption, ResolvedValue, UserId};

use crate::{cmd::RequestError, db::NavalVictoryCount, discord::ExecutionContext};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locator {
    Me,
    Bottom,
    Top,
    Someone(UserId),
    Rank(i64),
}

#[derive(Debug)]
pub struct Request<'a> {
    limit: i64,
    at: Locator,
    standin: PhantomData<&'a ()>,
}

impl<'a> Request<'a> {
    pub fn parse(_cmd: &'a CommandInteraction, options: &'_ [ResolvedOption<'a>]) -> Result<Self, RequestError> {
        let mut limit = 10;
        let mut at_discrim = "top";
        let mut rank = None;
        let mut someone = None;
        for opt in options {
            match opt.name {
                "limit" => {
                    let ResolvedValue::Integer(lim) = opt.value else {
                        trc::error!("Bad value for `limit` in `navy victory scoreboard` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `limit` in `navy victory scoreboard`.".into()));
                    };
                    if lim > 10 {
                        trc::error!("Bad value for `limit` in `legion kill scoreboard` {:?}", opt);
                        return Err(RequestError::User("You can only show 30 users per command.".into()));
                    }
                    limit = lim;
                },
                "at" => {
                    let ResolvedValue::String(a) = opt.value else {
                        trc::error!("Bad value for `at` in `navy victory scoreboard` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `at` in `navy victory scoreboard`".into()));
                    };
                    at_discrim = a;
                },
                "someone" => {
                    let ResolvedValue::User(u, _) = opt.value else {
                        trc::error!("Bad value for `someone` in `navy victory scoreboard` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `someone` in `navy victory scoreboard`".into()));
                    };
                    someone = Some(u.id);
                },
                "rank" => {
                    let ResolvedValue::Integer(r) = opt.value else {
                        trc::error!("Bad value for `rank` in `navy victory scoreboard` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `rank` in `navy victory scoreboard`".into()));
                    };
                    rank = Some(r);
                },
                _ => {
                    trc::error!("Unknown option in `navy victory scoreboard` {:?}", opt);
                    return Err(RequestError::Internal("Unknown value for `rank` in `navy victory scoreboard`".into()));
                }
            }
        }
        if rank.is_some() && at_discrim != "rank"{
            // Better error handling.
            return Err(RequestError::User("`rank` provided without providing `rank` for `at`".into()));
        }
        if someone.is_some() && at_discrim != "someone"{
            // Better error handling.
            return Err(RequestError::User("`someone` provided without providing `someone` for `at`".into()));
        }

        let at = match at_discrim {
            "me" => {
                Locator::Me
            },
            "bottom" => {
                Locator::Bottom
            },
            "top" => {
                Locator::Top
            },
            "someone" => {
                let Some(s) = someone else {
                    return Err(RequestError::User("`someone` is missing but `someone` provided for `at`.".into()));
                };
                Locator::Someone(s)
            },
            "rank" => {
                let Some(r) = rank else {
                    return Err(RequestError::User("`rank` is missing but `rank` provided for `at`.".into()));
                };
                Locator::Rank(r)
            },
            _ => {
                return Err(RequestError::Internal("Unknown value for `at` in `navy victory scoreboard`.".into()));
            },
        };

        Ok(Self {
            limit,
            at,
            standin: PhantomData
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        if self.limit == 0 {
            return ctx.reply("Scoreboard:".to_owned()).await;
        }

        let (start, ordering) = match self.at {
            Locator::Top => {
                let v = match NavalVictoryCount::load_asc(&ctx.db_cfg, 0, self.limit) {
                    Ok(v) => v,
                    Err(e) => {
                        trc::error!("Failed to get scoreboard items from top due to {e:?}.");
                        return Err(RequestError::Internal("failed to get scoreboard items".into()));
                    },
                };
                (1, v)
            },
            Locator::Bottom => {
                let count = match NavalVictoryCount::count_rows(&ctx.db_cfg) {
                    Ok(c) => c,
                    Err(e) => {
                        trc::error!("Failed to get scoreboard items from top due to {e:?}.");
                        return Err(RequestError::Internal("failed to get number of users".into()));
                    },
                };
                let mut a = match NavalVictoryCount::load_desc(&ctx.db_cfg, 0, self.limit) {
                    Ok(v) => v,
                    Err(e) => {
                        trc::error!("Failed to get scoreboard items from top due to {e:?}.");
                        return Err(RequestError::Internal("failed to get scoreboard items".into()));
                    },
                };
                a.reverse();
                (count - a.len() as i64 + 1, a)
            },
            Locator::Rank(r) => {
                let a = match NavalVictoryCount::load_asc(&ctx.db_cfg, r, self.limit) {
                    Ok(v) => v,
                    Err(e) => {
                        trc::error!("Failed to get scoreboard items from top due to {e:?}.");
                        return Err(RequestError::Internal("failed to get scoreboard items".into()));
                    },
                };
                (r + 1, a)
            },
            Locator::Me => {
                let rank = match NavalVictoryCount::get_rank_of(&ctx.db_cfg, ctx.cmd.user.id) {
                    Ok(r) => r,
                    Err(e) => {
                        trc::error!("Failed to get scoreboard items from me {:?} due to {e:?}.", ctx.cmd.user.id);
                        return Err(RequestError::Internal("failed to get scoreboard items".into()));
                    },
                };
                let start = 0.max(rank - (self.limit / 2));
                let v = match NavalVictoryCount::load_asc(&ctx.db_cfg, start, self.limit) {
                    Ok(v) => v,
                    Err(e) => {
                        trc::error!("Failed to get scoreboard items from top due to {e:?}.");
                        return Err(RequestError::Internal("failed to get scoreboard items".into()));
                    },
                };
                (start + 1, v)
            },
            Locator::Someone(u) => {
                let rank = match NavalVictoryCount::get_rank_of(&ctx.db_cfg, u) {
                    Ok(r) => r,
                    Err(e) => {
                        trc::error!("Failed to get scoreboard items from me {:?} due to {e:?}.", u);
                        return Err(RequestError::Internal("failed to get scoreboard items".into()));
                    },
                };
                let start = 0.max(rank - (self.limit / 2));
                let v = match NavalVictoryCount::load_asc(&ctx.db_cfg, start, self.limit) {
                    Ok(v) => v,
                    Err(e) => {
                        trc::error!("Failed to get scoreboard items from top due to {e:?}.");
                        return Err(RequestError::Internal("failed to get scoreboard items".into()));
                    },
                };
                (start + 1, v)
            },
        };

        let mut buffer = "**Scoreboard:**\n".to_owned();
        for (offset, record) in ordering.into_iter().enumerate() {
            buffer.push_str(format!(
                "\t{}) {}: {}",
                start + offset as i64,
                Mention::User(UserId::from(record.id.to_u64().unwrap())),
                record.victory_fourths.to_u64().unwrap() as f64 / 4.
            ).as_str());
            if record.victory_fourths.to_u64().unwrap() != 4 {
                buffer.push_str(" victories\n");
            } else {
                buffer.push_str(" victory\n");
            }
        }

        let _ = ctx.reply_restricted(buffer).await;
        Ok(())
    }
}
