use std::ops::Add;

use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use diesel::{BoolExpressionMethods, ConnectionError, ExpressionMethods, OptionalExtension, QueryDsl, prelude::{Identifiable, Insertable, Queryable}};
use diesel_async::RunQueryDsl;
use serenity::all::{GuildId, UserId};
use crate::schema;

use azel::db::{Connector, DbResult};

#[derive(Debug, Clone)]
#[derive(Insertable)]
#[diesel(table_name = schema::industry_profit_count_changes)]
pub struct NewIndustryProfitCountChange {
    pub updater: BigDecimal,
    pub target: BigDecimal,
    pub alpha_united_earth_credits: BigDecimal,
    pub guild_id: BigDecimal,
}

#[derive(Debug, Clone)]
#[derive(Insertable, Queryable, Identifiable)]
#[diesel(table_name = schema::industry_profit_counts)]
pub struct IndustryProfitCount {
    pub user_id: BigDecimal,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub alpha_united_earth_credits: BigDecimal,
    pub guild_id: BigDecimal,
    pub id: i64,
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

impl IndustryProfitCount {
    pub async fn load_for(connection_maker: &impl Connector, user_id: UserId, guild_id: GuildId) -> Option<Self> {
        let mut conn = connection_maker.async_connect().await.ok()?;
        schema::industry_profit_counts::table
            .filter(schema::industry_profit_counts::user_id.eq(BigDecimal::from(u64::from(user_id))))
            .filter(schema::industry_profit_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .get_result(&mut conn)
            .await
            .optional()
            .expect("query to be fine")
    }

    pub async fn adjust_count(connection_maker: &impl Connector, change: NewIndustryProfitCountChange) -> Result<BigDecimal, AdjustmentError> {
        let mut conn = connection_maker.async_connect().await.map_err(AdjustmentError::Connect)?;
        diesel::insert_into(schema::industry_profit_count_changes::table)
            .values(&change)
            .execute(&mut conn)
            .await
            .map_err(AdjustmentError::Change)?;
        diesel::insert_into(schema::industry_profit_counts::table)
            .values((
                schema::industry_profit_counts::user_id.eq(change.target),
                schema::industry_profit_counts::guild_id.eq(change.guild_id),
                schema::industry_profit_counts::updated.eq(diesel::dsl::now),
                schema::industry_profit_counts::alpha_united_earth_credits.eq(&change.alpha_united_earth_credits),
            ))
            .on_conflict((schema::industry_profit_counts::user_id, schema::industry_profit_counts::guild_id))
            .do_update()
            .set((
                schema::industry_profit_counts::updated.eq(diesel::dsl::now),
                schema::industry_profit_counts::alpha_united_earth_credits.eq(schema::industry_profit_counts::alpha_united_earth_credits.add(&change.alpha_united_earth_credits)),
            ))
            .returning(schema::industry_profit_counts::alpha_united_earth_credits)
            .get_result(&mut conn)
            .await
            .map_err(AdjustmentError::Count)
    }

    pub async fn count_rows(connection_maker: &impl Connector, guild_id: GuildId) -> DbResult<i64> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::industry_profit_counts::table.filter(schema::industry_profit_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id)))).count().get_result(&mut conn).await?)
    }

    pub async fn get_rank_of(connection_maker: &impl Connector, user_id: UserId, guild_id: GuildId) -> DbResult<i64> {
        let mut conn = connection_maker.async_connect().await?;
        let user_record = Self::load_for(connection_maker, user_id, guild_id).await;
        let alpha_united_earth_credits = user_record.as_ref().map(|r| r.alpha_united_earth_credits.clone()).unwrap_or_default();
        let usage = user_record.as_ref().map(|r| r.updated).unwrap_or(Utc::now() + Duration::milliseconds(100));
        Ok(schema::industry_profit_counts::table
            .filter(schema::industry_profit_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .filter(
                schema::industry_profit_counts::updated.lt(usage)
                    .and(schema::industry_profit_counts::alpha_united_earth_credits.gt(alpha_united_earth_credits))
            )
            .select(diesel::dsl::count(schema::industry_profit_counts::id))
            .get_result::<i64>(&mut conn)
            .await?)
    }

    pub async fn load_asc(connection_maker: &impl Connector, guild_id: GuildId, start: i64, lim: i64) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::industry_profit_counts::table
            .filter(schema::industry_profit_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .order((schema::industry_profit_counts::alpha_united_earth_credits.desc(), schema::industry_profit_counts::updated))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
            .await?)
    }

    pub async fn load_desc(connection_maker: &impl Connector, guild_id: GuildId, start: i64, lim: i64) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::industry_profit_counts::table
            .filter(schema::industry_profit_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .order((schema::industry_profit_counts::alpha_united_earth_credits, schema::industry_profit_counts::updated.desc()))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
            .await?)
    }

    pub async fn delete(connection_maker: &impl Connector, deleter: UserId, ids: &[i64]) -> Result<usize, AdjustmentError> {
        let mut conn = connection_maker.async_connect().await.map_err(AdjustmentError::Connect)?;
        let data = diesel::delete(
            schema::industry_profit_counts::table
                .filter(schema::industry_profit_counts::id.eq_any(ids))
        ).get_results::<Self>(&mut conn).await.map_err(AdjustmentError::Change)?;
        let deleted_record_count = data.len();

        // write changes back to db
        diesel::insert_into(schema::industry_profit_count_changes::table)
            .values(data.into_iter().map(|IndustryProfitCount { user_id, guild_id, alpha_united_earth_credits, .. }| NewIndustryProfitCountChange {
                updater: u64::from(deleter).into(),
                target: user_id,
                alpha_united_earth_credits,
                guild_id,
            }).collect::<Vec<_>>())
            .execute(&mut conn)
            .await
            .map_err(AdjustmentError::Count)?;

        Ok(deleted_record_count)
    }
}

