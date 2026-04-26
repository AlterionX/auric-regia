use std::ops::Add;

use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use diesel::{ConnectionError, ExpressionMethods, OptionalExtension, QueryDsl, prelude::{Identifiable, Insertable, Queryable}};
use diesel_async::RunQueryDsl;

use crate::{db::{DiscordGuildId, DiscordUserId}, schema};

use azel::db::{Connector, DbResult};

mod tracker_stat {
    use std::str::FromStr;

    use bigdecimal::BigDecimal;
    use diesel::{deserialize::FromSqlRow, expression::AsExpression, pg::Pg, sql_types::Text};
    use diesel_pg_type_utils::impl_sql_convert;
    use strum::{EnumIter, EnumString, IntoStaticStr};

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[derive(IntoStaticStr, EnumString, EnumIter)]
    #[derive(AsExpression, FromSqlRow)]
    #[diesel(sql_type = Text)]
    pub enum TrackerStat {
        #[strum(serialize = "industry_personnel_saved")]
        PersonnelSaved,
        #[strum(serialize = "event_participation")]
        EventParticipation,
        #[strum(serialize = "industry_auec")]
        IndustryAuec,
        #[strum(serialize = "human_kill")]
        GroundKill,
        #[strum(serialize = "navy_victory")]
        NavyVictory,
        #[strum(serialize = "tackle_assist")]
        NavyTackleAssist,
    }

    impl AsRef<str> for TrackerStat {
        fn as_ref(&self) -> &str {
            // IntoStaticStr should handle this
            self.into()
        }
    }

    impl TrackerStat {
        pub fn is_monthly_goal(&self) -> bool {
            match self {
                Self::PersonnelSaved => true,
                Self::EventParticipation => false,
                Self::IndustryAuec => false,
                Self::GroundKill => false,
                Self::NavyVictory => false,
                Self::NavyTackleAssist => false,
            }
        }

        pub fn as_str(&self) -> &'static str {
            self.into()
        }

        pub fn as_command_opt_display_name(&self) -> &'static str {
            match self {
                Self::PersonnelSaved => "Personnel Saved",
                Self::EventParticipation => "Event Participation",
                Self::IndustryAuec => "Industry Profit",
                Self::GroundKill => "Ground Kill",
                Self::NavyVictory => "Navy Victory",
                Self::NavyTackleAssist => "Navy Tackle Assist",
            }
        }

        pub fn denominator(&self) -> BigDecimal {
            match self {
                Self::NavyVictory => 4,
                _ => 1,
            }.into()
        }

        pub fn format_count(&self, db_value: BigDecimal) -> String {
            let display_value = db_value / self.denominator();
            let singular = display_value.is_one_quickcheck().unwrap_or(false);
            let counter_text = match self {
                Self::PersonnelSaved => "personnel saved",
                Self::EventParticipation => if singular {
                    "event"
                } else {
                    "events"
                },
                Self::IndustryAuec => if singular {
                    "credit"
                } else {
                    "credits"
                },
                Self::GroundKill => if singular {
                    "kill"
                } else {
                    "kills"
                },
                Self::NavyVictory => if singular {
                    "victory"
                } else {
                    "victories"
                },
                Self::NavyTackleAssist => if singular {
                    "tackle assist"
                } else {
                    "tackle assists"
                },
            };

            format!("{:.2} {}", display_value, counter_text)
        }

        pub fn format_count_as_past_participle(&self, db_value: BigDecimal) -> String {
            let display_value = db_value / self.denominator();
            let singular = display_value.is_one_quickcheck().unwrap_or(false);
            let (past_participle, counter_text) = match self {
                Self::PersonnelSaved => ("saved", "personnel"),
                Self::EventParticipation => if singular {
                    ("participated in ", "event")
                } else {
                    ("participated in ", "events")
                },
                Self::IndustryAuec => if singular {
                    ("contributed ", "credit")
                } else {
                    ("contributed ", "credits")
                },
                Self::GroundKill => if singular {
                    ("", "confirmed kill")
                } else {
                    ("", "confirmed kills")
                },
                Self::NavyVictory => if singular {
                    ("earned ", "victory")
                } else {
                    ("earned ", "victories")
                },
                Self::NavyTackleAssist => if singular {
                    ("earned ", "tackle assist")
                } else {
                    ("earned ", "tackle assists")
                },
            };

            format!("{}{:.2} {}", past_participle, display_value, counter_text)
        }

        pub fn display_value(&self, db_value: BigDecimal) -> BigDecimal {
            db_value / self.denominator()
        }

        pub fn db_value(&self, display_value: BigDecimal) -> BigDecimal {
            (display_value * self.denominator()).round(0)
        }
    }

    impl_sql_convert!(
        <Pg>
        Text > String > TrackerStat
        |s| {
            TrackerStat::from_str(s.as_str())
                .ok().ok_or("bad value")?
        }
        |scope| {
            &scope.as_ref().to_owned()
        }
    );

    impl TrackerStat {
        pub fn cmd_name(&self) -> &'static str {
            match self {
                Self::PersonnelSaved => "monthly_goal progress",
                Self::EventParticipation => "events participation",
                Self::IndustryAuec => "industry profit",
                Self::GroundKill => "ground kill",
                Self::NavyVictory => "navy victory",
                Self::NavyTackleAssist => "navy tackle_assist",
            }
        }

        pub fn default_add_remove_total(&self) -> BigDecimal {
            match self {
                Self::PersonnelSaved => 1,
                Self::EventParticipation => 1,
                Self::IndustryAuec => 1,
                Self::GroundKill => 1,
                Self::NavyVictory => 4,
                Self::NavyTackleAssist => 1,
            }.into()
        }
    }

    #[cfg(test)]
    mod test {
        use std::str::FromStr;

        #[test]
        fn test_stat_serde() {
            assert_eq!("industry_personnel_saved", <&'static str>::from(super::TrackerStat::PersonnelSaved));
            assert_eq!(Ok(super::TrackerStat::PersonnelSaved), super::TrackerStat::from_str("industry_personnel_saved"));
        }
    }
}
pub use tracker_stat::TrackerStat;

mod tracker_count_id {
    use diesel::pg::Pg;
    use diesel_pg_type_utils::wrap_i64;

    wrap_i64!(TrackerCountId<Pg>);
}
pub use tracker_count_id::TrackerCountId;

#[derive(Debug, Clone)]
#[derive(Insertable)]
#[diesel(table_name = schema::tracker_count_changes)]
pub struct NewTrackerCountChange {
    pub stat: TrackerStat,
    pub guild_id: DiscordGuildId,
    pub updater: DiscordUserId,
    pub target: DiscordUserId,
    pub total: BigDecimal,
    pub user_note: Option<String>,
}

#[derive(Debug, Clone)]
#[derive(Insertable, Queryable, Identifiable)]
#[diesel(table_name = schema::tracker_counts)]
pub struct TrackerCount {
    pub id: TrackerCountId,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub stat: TrackerStat,
    pub guild_id: DiscordGuildId,
    pub user_id: DiscordUserId,
    pub total: BigDecimal,
}

diesel::define_sql_function! {
    #[sql_name = "GREATEST"]
    fn max2(a: diesel::sql_types::Numeric, b: diesel::sql_types::Numeric) -> diesel::sql_types::Numeric;
}

#[derive(Debug, PartialEq)]
pub enum AdjustmentError {
    Connect(ConnectionError),
    Change(diesel::result::Error),
    Count(diesel::result::Error),
}

impl TrackerCount {
    pub async fn load_for(connection_maker: &impl Connector, stat: TrackerStat, guild_id: DiscordGuildId, user_id: DiscordUserId) -> Option<Self> {
        let mut conn = connection_maker.async_connect().await.ok()?;
        schema::tracker_counts::table
            .filter(schema::tracker_counts::stat.eq(stat))
            .filter(schema::tracker_counts::user_id.eq(user_id))
            .filter(schema::tracker_counts::guild_id.eq(guild_id))
            .get_result(&mut conn)
            .await
            .optional()
            .expect("query to be fine")
    }

    pub async fn adjust_count(connection_maker: &impl Connector, change: NewTrackerCountChange) -> Result<BigDecimal, AdjustmentError> {
        let mut conn = connection_maker.async_connect().await
            .map_err(AdjustmentError::Connect)?;
        diesel::insert_into(schema::tracker_count_changes::table)
            .values(&change)
            .execute(&mut conn)
            .await
            .map_err(AdjustmentError::Change)?;
        diesel::insert_into(schema::tracker_counts::table)
            .values((
                schema::tracker_counts::stat.eq(change.stat),
                schema::tracker_counts::user_id.eq(change.target),
                schema::tracker_counts::guild_id.eq(change.guild_id),
                schema::tracker_counts::updated.eq(diesel::dsl::now),
                schema::tracker_counts::total.eq(max2(
                    BigDecimal::from(0),
                    &change.total,
                )),
            ))
            .on_conflict((schema::tracker_counts::stat, schema::tracker_counts::guild_id, schema::tracker_counts::user_id))
            .do_update()
            .set((
                schema::tracker_counts::updated.eq(diesel::dsl::now),
                schema::tracker_counts::total.eq(max2(
                    BigDecimal::from(0),
                    schema::tracker_counts::total.add(&change.total)
                )),
            ))
            .returning(schema::tracker_counts::total)
            .get_result(&mut conn)
            .await
            .map_err(AdjustmentError::Count)
    }

    pub async fn delete(connection_maker: &impl Connector, deleter: DiscordUserId, ids: &[TrackerCountId]) -> Result<usize, AdjustmentError> {
        let mut conn = connection_maker.async_connect().await.map_err(AdjustmentError::Connect)?;
        let data = diesel::delete(
            schema::tracker_counts::table
                .filter(schema::tracker_counts::id.eq_any(ids))
        ).get_results::<Self>(&mut conn).await.map_err(AdjustmentError::Change)?;
        let deleted_record_count = data.len();

        // write changes back to db
        diesel::insert_into(schema::tracker_count_changes::table)
            .values(data.into_iter().map(|TrackerCount { stat, user_id, guild_id, total, .. }| NewTrackerCountChange {
                stat,
                updater: deleter,
                target: user_id,
                total: -total,
                guild_id,
                user_note: None,
            }).collect::<Vec<_>>())
            .execute(&mut conn)
            .await
            .map_err(AdjustmentError::Count)?;

        Ok(deleted_record_count)
    }

    pub async fn count_rows(connection_maker: &impl Connector, stat: TrackerStat, guild_id: DiscordGuildId) -> DbResult<i64> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::tracker_counts::table
            .filter(schema::tracker_counts::stat.eq(stat))
            .filter(schema::tracker_counts::guild_id.eq(guild_id))
            .count()
            .get_result(&mut conn)
            .await?)
    }

    pub async fn get_rank_of(connection_maker: &impl Connector, stat: TrackerStat, guild_id: DiscordGuildId, user_id: DiscordUserId) -> DbResult<i64> {
        let mut conn = connection_maker.async_connect().await?;
        let user_record = Self::load_for(connection_maker, stat, guild_id, user_id).await;
        let total = user_record.as_ref().map(|r| r.total.clone()).unwrap_or_default();
        let usage = user_record.as_ref().map(|r| r.updated).unwrap_or(Utc::now() + Duration::milliseconds(100));
        Ok(schema::tracker_counts::table
            .filter(schema::tracker_counts::stat.eq(stat))
            .filter(schema::tracker_counts::guild_id.eq(guild_id))
            .filter(schema::tracker_counts::updated.lt(usage))
            .filter(schema::tracker_counts::total.gt(total))
            .select(diesel::dsl::count(schema::tracker_counts::id))
            .get_result::<i64>(&mut conn)
            .await?)
    }

    pub async fn load_asc(connection_maker: &impl Connector, stat: TrackerStat, guild_id: DiscordGuildId, start: i64, lim: i64) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::tracker_counts::table
            .filter(schema::tracker_counts::stat.eq(stat))
            .filter(schema::tracker_counts::guild_id.eq(guild_id))
            .order_by((schema::tracker_counts::total.desc(), schema::tracker_counts::updated))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
            .await?)
    }

    pub async fn load_desc(connection_maker: &impl Connector, stat: TrackerStat, guild_id: DiscordGuildId, start: i64, lim: i64) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::tracker_counts::table
            .filter(schema::tracker_counts::stat.eq(stat))
            .filter(schema::tracker_counts::guild_id.eq(guild_id))
            .order_by((schema::tracker_counts::total, schema::tracker_counts::updated.desc()))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
            .await?)
    }
}
