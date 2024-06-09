use std::ops::Add;

use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use serenity::all::UserId;
use crate::schema::{self, event_participation_counts};

use super::Connector;

#[derive(Debug, Clone)]
#[derive(Insertable)]
#[diesel(table_name = schema::event_participation_count_changes)]
pub struct NewEventParticipationCountChange {
    pub updater: BigDecimal,
    pub target: BigDecimal,
    pub event_participation: BigDecimal,
    pub user_note: Option<String>,
}

#[derive(Debug, Clone)]
#[derive(Insertable, Queryable, Identifiable)]
#[diesel(table_name = schema::event_participation_counts)]
pub struct EventParticipationCount {
    pub id: BigDecimal,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub event_participation: BigDecimal,
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

impl EventParticipationCount {
    pub fn load_for(connection_maker: &impl Connector, user: UserId) -> Option<Self> {
        let mut conn = connection_maker.connect();
        let id: u64 = user.into();
        schema::event_participation_counts::table
            .filter(schema::event_participation_counts::id.eq(BigDecimal::from(id)))
            .get_result(&mut conn)
            .optional()
            .expect("query to be fine")
    }

    pub fn adjust_count(connection_maker: &impl Connector, change: NewEventParticipationCountChange) -> Result<BigDecimal, AdjustmentError> {
        let mut conn = connection_maker.connect();
        diesel::insert_into(schema::event_participation_count_changes::table)
            .values(&change)
            .execute(&mut conn)
            .map_err(AdjustmentError::Change)?;
        diesel::insert_into(schema::event_participation_counts::table)
            .values((
                schema::event_participation_counts::id.eq(change.target),
                schema::event_participation_counts::updated.eq(diesel::dsl::now),
                schema::event_participation_counts::event_participation.eq(max2(
                    BigDecimal::from(0),
                    &change.event_participation,
                )),
            ))
            .on_conflict(schema::event_participation_counts::id)
            .do_update()
            .set((
                schema::event_participation_counts::updated.eq(diesel::dsl::now),
                schema::event_participation_counts::event_participation.eq(max2(
                    BigDecimal::from(0),
                    schema::event_participation_counts::event_participation.add(&change.event_participation)
                )),
            ))
            .returning(schema::event_participation_counts::event_participation)
            .get_result(&mut conn)
            .map_err(AdjustmentError::Count)
    }

    pub fn count_rows(connection_maker: &impl Connector) -> Result<i64, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        schema::event_participation_counts::table.count().get_result(&mut conn)
    }

    pub fn get_rank_of(connection_maker: &impl Connector, user_id: UserId) -> Result<i64, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        let user_record = Self::load_for(connection_maker, user_id);
        let event_participation = user_record.as_ref().map(|r| r.event_participation.clone()).unwrap_or_default();
        let usage = user_record.as_ref().map(|r| r.updated).unwrap_or(Utc::now() + Duration::milliseconds(100));
        schema::event_participation_counts::table
            .filter(
                schema::event_participation_counts::updated.lt(usage)
                    .and(schema::event_participation_counts::event_participation.gt(event_participation))
            )
            .select(diesel::dsl::count(event_participation_counts::id))
            .get_result::<i64>(&mut conn)
    }

    pub fn load_asc(connection_maker: &impl Connector, start: i64, lim: i64) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        schema::event_participation_counts::table
            .order((event_participation_counts::event_participation.desc(), event_participation_counts::updated))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
    }

    pub fn load_desc(connection_maker: &impl Connector, start: i64, lim: i64) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        schema::event_participation_counts::table
            .order((event_participation_counts::event_participation, event_participation_counts::updated.desc()))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
    }
}

