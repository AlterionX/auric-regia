use std::ops::Add;

use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use serenity::all::UserId;
use crate::schema::{self, legion_kill_counts};

use super::Connector;

#[derive(Debug, Clone)]
#[derive(Insertable)]
#[diesel(table_name = schema::legion_kill_count_changes)]
pub struct NewLegionKillCountChange {
    pub updater: BigDecimal,
    pub target: BigDecimal,
    pub kills: BigDecimal,
}

#[derive(Debug, Clone)]
#[derive(Insertable, Queryable, Identifiable)]
#[diesel(table_name = schema::legion_kill_counts)]
pub struct LegionKillCount {
    pub id: BigDecimal,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub kills: BigDecimal,
}

diesel::define_sql_function! {
    #[sql_name = "GREATEST"]
    fn max2(a: diesel::sql_types::Numeric, b: diesel::sql_types::Numeric) -> diesel::sql_types::Numeric;
}

#[derive(Debug, PartialEq)]
pub enum AdjustmentError {
    Change(diesel::result::Error),
    Count(diesel::result::Error),
}

impl LegionKillCount {
    pub fn load_for(connection_maker: &impl Connector, user: UserId) -> Option<Self> {
        let mut conn = connection_maker.connect();
        let id: u64 = user.into();
        schema::legion_kill_counts::table
            .filter(schema::legion_kill_counts::id.eq(BigDecimal::from(id)))
            .get_result(&mut conn)
            .optional()
            .expect("query to be fine")
    }

    pub fn adjust_count(connection_maker: &impl Connector, change: NewLegionKillCountChange) -> Result<BigDecimal, AdjustmentError> {
        let mut conn = connection_maker.connect();
        diesel::insert_into(schema::legion_kill_count_changes::table)
            .values(&change)
            .execute(&mut conn)
            .map_err(AdjustmentError::Change)?;
        diesel::insert_into(schema::legion_kill_counts::table)
            .values((
                schema::legion_kill_counts::id.eq(change.target),
                schema::legion_kill_counts::updated.eq(diesel::dsl::now),
                schema::legion_kill_counts::kills.eq(max2(
                    BigDecimal::from(0),
                    &change.kills,
                )),
            ))
            .on_conflict(schema::legion_kill_counts::id)
            .do_update()
            .set((
                schema::legion_kill_counts::updated.eq(diesel::dsl::now),
                schema::legion_kill_counts::kills.eq(max2(
                    BigDecimal::from(0),
                    schema::legion_kill_counts::kills.add(&change.kills)
                )),
            ))
            .returning(schema::legion_kill_counts::kills)
            .get_result(&mut conn)
            .map_err(AdjustmentError::Count)
    }

    pub fn count_rows(connection_maker: &impl Connector) -> Result<i64, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        schema::legion_kill_counts::table.count().get_result(&mut conn)
    }

    pub fn get_rank_of(connection_maker: &impl Connector, user_id: UserId) -> Result<i64, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        let user_record = Self::load_for(connection_maker, user_id);
        let kills = user_record.as_ref().map(|r| r.kills.clone()).unwrap_or_default();
        let usage = user_record.as_ref().map(|r| r.updated).unwrap_or(Utc::now() + Duration::milliseconds(100));
        schema::legion_kill_counts::table
            .filter(
                schema::legion_kill_counts::updated.lt(usage)
                    .and(schema::legion_kill_counts::kills.gt(kills))
            )
            .select(diesel::dsl::count(legion_kill_counts::id))
            .get_result::<i64>(&mut conn)
    }

    pub fn load_asc(connection_maker: &impl Connector, start: i64, lim: i64) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        schema::legion_kill_counts::table
            .order((legion_kill_counts::kills.desc(), legion_kill_counts::updated))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
    }

    pub fn load_desc(connection_maker: &impl Connector, start: i64, lim: i64) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        schema::legion_kill_counts::table
            .order((legion_kill_counts::kills, legion_kill_counts::updated.desc()))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
    }
}
