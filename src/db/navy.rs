use std::ops::Add;

use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use serenity::all::UserId;
use crate::schema;

use super::Connector;

#[derive(Debug, PartialEq)]
pub enum AdjustmentError {
    Change(diesel::result::Error),
    Count(diesel::result::Error),
}

#[derive(Debug, Clone)]
#[derive(Insertable)]
#[diesel(table_name = schema::naval_victory_count_changes)]
pub struct NewNavalVictoryCountChange {
    pub updater: BigDecimal,
    pub target: BigDecimal,
    pub victory_fourths: BigDecimal,
}

#[derive(Debug, Clone)]
#[derive(Insertable, Queryable, Identifiable)]
#[diesel(table_name = schema::naval_victory_counts)]
pub struct NavalVictoryCount {
    pub id: BigDecimal,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub victory_fourths: BigDecimal,
}

diesel::define_sql_function! {
    #[sql_name = "GREATEST"]
    fn max2(a: diesel::sql_types::Numeric, b: diesel::sql_types::Numeric) -> diesel::sql_types::Numeric;
}

impl NavalVictoryCount {
    pub fn load_for(connection_maker: &impl Connector, user: UserId) -> Option<Self> {
        let mut conn = connection_maker.connect();
        let id: u64 = user.into();
        schema::naval_victory_counts::table
            .filter(schema::naval_victory_counts::id.eq(BigDecimal::from(id)))
            .get_result(&mut conn)
            .optional()
            .expect("query to be fine")
    }

    pub fn adjust_count(connection_maker: &impl Connector, change: NewNavalVictoryCountChange) -> Result<BigDecimal, AdjustmentError> {
        let mut conn = connection_maker.connect();
        diesel::insert_into(schema::naval_victory_count_changes::table)
            .values(&change)
            .execute(&mut conn)
            .map_err(AdjustmentError::Change)?;
        diesel::insert_into(schema::naval_victory_counts::table)
            .values((
                schema::naval_victory_counts::id.eq(change.target),
                schema::naval_victory_counts::updated.eq(diesel::dsl::now),
                schema::naval_victory_counts::victory_fourths.eq(max2(
                    BigDecimal::from(0),
                    &change.victory_fourths,
                )),
            ))
            .on_conflict(schema::naval_victory_counts::id)
            .do_update()
            .set((
                schema::naval_victory_counts::updated.eq(diesel::dsl::now),
                schema::naval_victory_counts::victory_fourths.eq(max2(
                    BigDecimal::from(0),
                    schema::naval_victory_counts::victory_fourths.add(&change.victory_fourths)
                )),
            ))
            .returning(schema::naval_victory_counts::victory_fourths)
            .get_result(&mut conn)
            .map_err(AdjustmentError::Count)
    }

    pub fn count_rows(connection_maker: &impl Connector) -> Result<i64, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        schema::naval_victory_counts::table.count().get_result(&mut conn)
    }

    pub fn get_rank_of(connection_maker: &impl Connector, user_id: UserId) -> Result<i64, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        let user_record = Self::load_for(connection_maker, user_id);
        let victory_fourths = user_record.as_ref().map(|r| r.victory_fourths.clone()).unwrap_or_default();
        let usage = user_record.as_ref().map(|r| r.updated).unwrap_or(Utc::now() + Duration::milliseconds(100));
        schema::naval_victory_counts::table
            .filter(
                schema::naval_victory_counts::updated.lt(usage)
                    .and(schema::naval_victory_counts::victory_fourths.gt(victory_fourths))
            )
            .select(diesel::dsl::count(schema::naval_victory_counts::id))
            .get_result::<i64>(&mut conn)
    }

    pub fn load_asc(connection_maker: &impl Connector, start: i64, lim: i64) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        schema::naval_victory_counts::table
            .order((schema::naval_victory_counts::victory_fourths.desc(), schema::naval_victory_counts::updated))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
    }

    pub fn load_desc(connection_maker: &impl Connector, start: i64, lim: i64) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        schema::naval_victory_counts::table
            .order((schema::naval_victory_counts::victory_fourths, schema::naval_victory_counts::updated.desc()))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
    }

    pub fn delete(connection_maker: &impl Connector, deleter: UserId, ids: &[BigDecimal]) -> Result<usize, AdjustmentError> {
        let mut conn = connection_maker.connect();
        let data = diesel::delete(
            schema::naval_victory_counts::table
                .filter(schema::naval_victory_counts::id.eq_any(ids))
        ).get_results::<Self>(&mut conn).map_err(AdjustmentError::Change)?;
        let deleted_record_count = data.len();

        // write changes back to db
        diesel::insert_into(schema::naval_victory_count_changes::table)
            .values(data.into_iter().map(|NavalVictoryCount { id, victory_fourths, .. }| NewNavalVictoryCountChange {
                updater: u64::from(deleter).into(),
                target: id,
                victory_fourths,
            }).collect::<Vec<_>>())
            .execute(&mut conn)
            .map_err(AdjustmentError::Count)?;

        Ok(deleted_record_count)
    }
}

#[derive(Debug, Clone)]
#[derive(Insertable)]
#[diesel(table_name = schema::naval_tackle_assist_count_changes)]
pub struct NewNavalTackleAssistCountChange {
    pub updater: BigDecimal,
    pub target: BigDecimal,
    pub tackle_assists: BigDecimal,
}

#[derive(Debug, Clone)]
#[derive(Insertable, Queryable, Identifiable)]
#[diesel(table_name = schema::naval_tackle_assist_counts)]
pub struct NavalTackleAssistCount {
    pub id: BigDecimal,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub tackle_assists: BigDecimal,
}


impl NavalTackleAssistCount {
    pub fn load_for(connection_maker: &impl Connector, user: UserId) -> Option<Self> {
        let mut conn = connection_maker.connect();
        let id: u64 = user.into();
        schema::naval_tackle_assist_counts::table
            .filter(schema::naval_tackle_assist_counts::id.eq(BigDecimal::from(id)))
            .get_result(&mut conn)
            .optional()
            .expect("query to be fine")
    }

    pub fn adjust_count(connection_maker: &impl Connector, change: NewNavalTackleAssistCountChange) -> Result<BigDecimal, AdjustmentError> {
        let mut conn = connection_maker.connect();
        diesel::insert_into(schema::naval_tackle_assist_count_changes::table)
            .values(&change)
            .execute(&mut conn)
            .map_err(AdjustmentError::Change)?;
        diesel::insert_into(schema::naval_tackle_assist_counts::table)
            .values((
                schema::naval_tackle_assist_counts::id.eq(change.target),
                schema::naval_tackle_assist_counts::updated.eq(diesel::dsl::now),
                schema::naval_tackle_assist_counts::tackle_assists.eq(max2(
                    BigDecimal::from(0),
                    &change.tackle_assists,
                )),
            ))
            .on_conflict(schema::naval_tackle_assist_counts::id)
            .do_update()
            .set((
                schema::naval_tackle_assist_counts::updated.eq(diesel::dsl::now),
                schema::naval_tackle_assist_counts::tackle_assists.eq(max2(
                    BigDecimal::from(0),
                    schema::naval_tackle_assist_counts::tackle_assists.add(&change.tackle_assists)
                )),
            ))
            .returning(schema::naval_tackle_assist_counts::tackle_assists)
            .get_result(&mut conn)
            .map_err(AdjustmentError::Count)
    }

    pub fn count_rows(connection_maker: &impl Connector) -> Result<i64, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        schema::naval_tackle_assist_counts::table.count().get_result(&mut conn)
    }

    pub fn get_rank_of(connection_maker: &impl Connector, user_id: UserId) -> Result<i64, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        let user_record = Self::load_for(connection_maker, user_id);
        let tackle_assists = user_record.as_ref().map(|r| r.tackle_assists.clone()).unwrap_or_default();
        let usage = user_record.as_ref().map(|r| r.updated).unwrap_or(Utc::now() + Duration::milliseconds(100));
        schema::naval_tackle_assist_counts::table
            .filter(
                schema::naval_tackle_assist_counts::updated.lt(usage)
                    .and(schema::naval_tackle_assist_counts::tackle_assists.gt(tackle_assists))
            )
            .select(diesel::dsl::count(schema::naval_tackle_assist_counts::id))
            .get_result::<i64>(&mut conn)
    }

    pub fn load_asc(connection_maker: &impl Connector, start: i64, lim: i64) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        schema::naval_tackle_assist_counts::table
            .order((schema::naval_tackle_assist_counts::tackle_assists.desc(), schema::naval_tackle_assist_counts::updated))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
    }

    pub fn load_desc(connection_maker: &impl Connector, start: i64, lim: i64) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        schema::naval_tackle_assist_counts::table
            .order((schema::naval_tackle_assist_counts::tackle_assists, schema::naval_tackle_assist_counts::updated.desc()))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
    }

    pub fn delete(connection_maker: &impl Connector, deleter: UserId, ids: &[BigDecimal]) -> Result<usize, AdjustmentError> {
        let mut conn = connection_maker.connect();
        let data = diesel::delete(
            schema::naval_tackle_assist_counts::table
                .filter(schema::naval_tackle_assist_counts::id.eq_any(ids))
        ).get_results::<Self>(&mut conn).map_err(AdjustmentError::Change)?;
        let deleted_record_count = data.len();

        // write changes back to db
        diesel::insert_into(schema::naval_tackle_assist_count_changes::table)
            .values(data.into_iter().map(|NavalTackleAssistCount { id, tackle_assists, .. }| NewNavalTackleAssistCountChange {
                updater: u64::from(deleter).into(),
                target: id,
                tackle_assists,
            }).collect::<Vec<_>>())
            .execute(&mut conn)
            .map_err(AdjustmentError::Count)?;

        Ok(deleted_record_count)
    }
}
