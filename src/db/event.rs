use std::ops::Add;

use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use diesel::{BoolExpressionMethods, ConnectionError, ExpressionMethods, OptionalExtension, QueryDsl, prelude::{Identifiable, Insertable, Queryable}};
use diesel_async::RunQueryDsl;
use serenity::all::UserId;
use crate::schema;

use azel::db::{Connector, DbResult};

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
    Connect(ConnectionError),
    Change(diesel::result::Error),
    Count(diesel::result::Error),
}

impl EventParticipationCount {
    pub async fn load_for(connection_maker: &impl Connector, user: UserId) -> Option<Self> {
        let mut conn = connection_maker.async_connect().await.ok()?;
        let id: u64 = user.into();
        schema::event_participation_counts::table
            .filter(schema::event_participation_counts::id.eq(BigDecimal::from(id)))
            .get_result(&mut conn)
            .await
            .optional()
            .expect("query to be fine")
    }

    pub async fn adjust_count(connection_maker: &impl Connector, change: NewEventParticipationCountChange) -> Result<BigDecimal, AdjustmentError> {
        let mut conn = connection_maker.async_connect().await
            .map_err(AdjustmentError::Connect)?;
        diesel::insert_into(schema::event_participation_count_changes::table)
            .values(&change)
            .execute(&mut conn)
            .await
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
            .await
            .map_err(AdjustmentError::Count)
    }

    pub async fn count_rows(connection_maker: &impl Connector) -> DbResult<i64> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::event_participation_counts::table.count().get_result(&mut conn).await?)
    }

    pub async fn get_rank_of(connection_maker: &impl Connector, user_id: UserId) -> DbResult<i64> {
        let mut conn = connection_maker.async_connect().await?;
        let user_record = Self::load_for(connection_maker, user_id).await;
        let event_participation = user_record.as_ref().map(|r| r.event_participation.clone()).unwrap_or_default();
        let usage = user_record.as_ref().map(|r| r.updated).unwrap_or(Utc::now() + Duration::milliseconds(100));
        Ok(schema::event_participation_counts::table
            .filter(
                schema::event_participation_counts::updated.lt(usage)
                    .and(schema::event_participation_counts::event_participation.gt(event_participation))
            )
            .select(diesel::dsl::count(schema::event_participation_counts::id))
            .get_result::<i64>(&mut conn)
            .await?)
    }

    pub async fn load_asc(connection_maker: &impl Connector, start: i64, lim: i64) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::event_participation_counts::table
            .order_by((schema::event_participation_counts::event_participation.desc(), schema::event_participation_counts::updated))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
            .await?)
    }

    pub async fn load_desc(connection_maker: &impl Connector, start: i64, lim: i64) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::event_participation_counts::table
            .order_by((schema::event_participation_counts::event_participation, schema::event_participation_counts::updated.desc()))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
            .await?)
    }
}

