use std::marker::PhantomData;
use bigdecimal::ToPrimitive;
use tracing as trc;

use serenity::all::{CommandInteraction, Mentionable, ResolvedOption, ResolvedValue};

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db::{self, DiscordGuildId, DiscordUserId, TrackerStat}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locator {
    Me,
    Bottom,
    Top,
    Someone(DiscordUserId),
    Rank(i64),
}

#[derive(Debug)]
pub struct Request<'a> {
    stat: TrackerStat,
    guild_id: DiscordGuildId,
    limit: i64,
    at: Locator,
    standin: PhantomData<&'a ()>,
}

impl<'a> Request<'a> {
    pub fn parse(cmd: &'a CommandInteraction, stat: TrackerStat, options: &'_ [ResolvedOption<'a>]) -> Result<Self, RequestError> {
        let guild_id = cmd.guild_id.ok_or_else(|| RequestError::User("Command must be run from within a guild.".into()))?.into();
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
                    if lim > 50 {
                        trc::error!("Bad value for `limit` in `legion kill scoreboard` {:?}", opt);
                        return Err(RequestError::User("You can only show 50 users per command.".into()));
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
        if rank.is_some() && at_discrim != "rank" {
            // Overrides `at_discrim` for convenience
            at_discrim = "rank";
        }
        if someone.is_some() && at_discrim != "someone" {
            // Overrides `at_discrim` for convenience
            at_discrim = "someone";
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
                Locator::Someone(s.into())
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
            stat,
            guild_id,
            limit,
            at,
            standin: PhantomData
        })
    }

    pub async fn execute(Self { stat, guild_id, limit, at, .. }: Self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        if limit == 0 {
            return ctx.reply("Scoreboard:".to_owned()).await;
        }

        let (start, ordering) = match at {
            Locator::Top => {
                let v = match db::TrackerCount::load_asc(&ctx.db_cfg, stat, guild_id, 0, limit).await {
                    Ok(v) => v,
                    Err(e) => {
                        trc::error!("Failed to get scoreboard items from top due to {e:?}.");
                        return Err(RequestError::Internal("failed to get scoreboard items".into()));
                    },
                };
                (1, v)
            },
            Locator::Bottom => {
                let count = match db::TrackerCount::count_rows(&ctx.db_cfg, stat, guild_id).await {
                    Ok(c) => c,
                    Err(e) => {
                        trc::error!("Failed to get scoreboard items from top due to {e:?}.");
                        return Err(RequestError::Internal("failed to get number of users".into()));
                    },
                };
                let mut a = match db::TrackerCount::load_desc(&ctx.db_cfg, stat, guild_id, 0, limit).await {
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
                let a = match db::TrackerCount::load_asc(&ctx.db_cfg, stat, guild_id, r, limit).await {
                    Ok(v) => v,
                    Err(e) => {
                        trc::error!("Failed to get scoreboard items from top due to {e:?}.");
                        return Err(RequestError::Internal("failed to get scoreboard items".into()));
                    },
                };
                (r + 1, a)
            },
            Locator::Me => {
                let rank = match db::TrackerCount::get_rank_of(&ctx.db_cfg, stat, guild_id, ctx.cmd.user.id.into()).await {
                    Ok(r) => r,
                    Err(e) => {
                        trc::error!("Failed to get scoreboard items from me {:?} due to {e:?}.", ctx.cmd.user.id);
                        return Err(RequestError::Internal("failed to get scoreboard items".into()));
                    },
                };
                let start = 0.max(rank - (limit / 2));
                let v = match db::TrackerCount::load_asc(&ctx.db_cfg, stat, guild_id, start, limit).await {
                    Ok(v) => v,
                    Err(e) => {
                        trc::error!("Failed to get scoreboard items from top due to {e:?}.");
                        return Err(RequestError::Internal("failed to get scoreboard items".into()));
                    },
                };
                (start + 1, v)
            },
            Locator::Someone(u) => {
                let rank = match db::TrackerCount::get_rank_of(&ctx.db_cfg, stat, guild_id, u.into()).await {
                    Ok(r) => r,
                    Err(e) => {
                        trc::error!("Failed to get scoreboard items from me {:?} due to {e:?}.", u);
                        return Err(RequestError::Internal("failed to get scoreboard items".into()));
                    },
                };
                let start = 0.max(rank - (limit / 2));
                let v = match db::TrackerCount::load_asc(&ctx.db_cfg, stat, guild_id, start, limit).await {
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
            append_row_for_stat(stat, start + offset as i64, record, &mut buffer);
        }

        ctx.reply_restricted(buffer).await
    }
}

fn append_row_for_stat(stat: TrackerStat, rank: i64, record: db::TrackerCount, buffer: &mut String) {
    match stat {
        TrackerStat::PersonnelSaved => {
            buffer.push_str(format!(
                "\t{}) {}: {} personnel saved\n",
                rank,
                record.user_id.inner().mention(),
                record.total.to_u64().unwrap()
            ).as_str());
        },
    }
}
