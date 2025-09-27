use std::ops::Add;

use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use serenity::all::UserId;
use crate::schema;

use super::Connector;

#[derive(Debug, Clone)]
#[derive(Insertable)]
#[diesel(table_name = schema::industry_profit_count_changes)]
pub struct NewIndustryProfitCountChange {
    pub updater: BigDecimal,
    pub target: BigDecimal,
    pub alpha_united_earth_credits: BigDecimal,
}

#[derive(Debug, Clone)]
#[derive(Insertable, Queryable, Identifiable)]
#[diesel(table_name = schema::industry_profit_counts)]
pub struct IndustryProfitCount {
    pub id: BigDecimal,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub alpha_united_earth_credits: BigDecimal,
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

impl IndustryProfitCount {
    pub fn load_for(connection_maker: &impl Connector, user: UserId) -> Option<Self> {
        let mut conn = connection_maker.connect();
        let id: u64 = user.into();
        schema::industry_profit_counts::table
            .filter(schema::industry_profit_counts::id.eq(BigDecimal::from(id)))
            .get_result(&mut conn)
            .optional()
            .expect("query to be fine")
    }

    pub fn adjust_count(connection_maker: &impl Connector, change: NewIndustryProfitCountChange) -> Result<BigDecimal, AdjustmentError> {
        let mut conn = connection_maker.connect();
        diesel::insert_into(schema::industry_profit_count_changes::table)
            .values(&change)
            .execute(&mut conn)
            .map_err(AdjustmentError::Change)?;
        diesel::insert_into(schema::industry_profit_counts::table)
            .values((
                schema::industry_profit_counts::id.eq(change.target),
                schema::industry_profit_counts::updated.eq(diesel::dsl::now),
                schema::industry_profit_counts::alpha_united_earth_credits.eq(&change.alpha_united_earth_credits),
            ))
            .on_conflict(schema::industry_profit_counts::id)
            .do_update()
            .set((
                schema::industry_profit_counts::updated.eq(diesel::dsl::now),
                schema::industry_profit_counts::alpha_united_earth_credits.eq(schema::industry_profit_counts::alpha_united_earth_credits.add(&change.alpha_united_earth_credits)),
            ))
            .returning(schema::industry_profit_counts::alpha_united_earth_credits)
            .get_result(&mut conn)
            .map_err(AdjustmentError::Count)
    }

    pub fn count_rows(connection_maker: &impl Connector) -> Result<i64, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        schema::industry_profit_counts::table.count().get_result(&mut conn)
    }

    pub fn get_rank_of(connection_maker: &impl Connector, user_id: UserId) -> Result<i64, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        let user_record = Self::load_for(connection_maker, user_id);
        let alpha_united_earth_credits = user_record.as_ref().map(|r| r.alpha_united_earth_credits.clone()).unwrap_or_default();
        let usage = user_record.as_ref().map(|r| r.updated).unwrap_or(Utc::now() + Duration::milliseconds(100));
        schema::industry_profit_counts::table
            .filter(
                schema::industry_profit_counts::updated.lt(usage)
                    .and(schema::industry_profit_counts::alpha_united_earth_credits.gt(alpha_united_earth_credits))
            )
            .select(diesel::dsl::count(schema::industry_profit_counts::id))
            .get_result::<i64>(&mut conn)
    }

    pub fn load_asc(connection_maker: &impl Connector, start: i64, lim: i64) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        schema::industry_profit_counts::table
            .order((schema::industry_profit_counts::alpha_united_earth_credits.desc(), schema::industry_profit_counts::updated))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
    }

    pub fn load_desc(connection_maker: &impl Connector, start: i64, lim: i64) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection_maker.connect();
        schema::industry_profit_counts::table
            .order((schema::industry_profit_counts::alpha_united_earth_credits, schema::industry_profit_counts::updated.desc()))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
    }

    pub fn delete(connection_maker: &impl Connector, deleter: UserId, ids: &[BigDecimal]) -> Result<usize, AdjustmentError> {
        let mut conn = connection_maker.connect();
        let data = diesel::delete(
            schema::industry_profit_counts::table
                .filter(schema::industry_profit_counts::id.eq_any(ids))
        ).get_results::<Self>(&mut conn).map_err(AdjustmentError::Change)?;
        let deleted_record_count = data.len();

        // write changes back to db
        diesel::insert_into(schema::industry_profit_count_changes::table)
            .values(data.into_iter().map(|IndustryProfitCount { id, alpha_united_earth_credits, .. }| NewIndustryProfitCountChange {
                updater: u64::from(deleter).into(),
                target: id,
                alpha_united_earth_credits,
            }).collect::<Vec<_>>())
            .execute(&mut conn)
            .map_err(AdjustmentError::Count)?;

        Ok(deleted_record_count)
    }
}

