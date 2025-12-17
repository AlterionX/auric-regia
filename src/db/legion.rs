use std::ops::Add;

use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use diesel::{BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, prelude::{Identifiable, Insertable, Queryable}};
use diesel_async::RunQueryDsl;
use serenity::all::UserId;
use crate::schema;

use azel::db::{Connector, DbResult};

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
    Connect(diesel::result::ConnectionError),
    Change(diesel::result::Error),
    Count(diesel::result::Error),
}

impl LegionKillCount {
    pub async fn load_for(connection_maker: &impl Connector, user: UserId) -> Option<Self> {
        let mut conn = connection_maker.async_connect().await.ok()?;
        let id: u64 = user.into();
        schema::legion_kill_counts::table
            .filter(schema::legion_kill_counts::id.eq(BigDecimal::from(id)))
            .get_result(&mut conn)
            .await
            .optional()
            .expect("query to be fine")
    }

    pub async fn adjust_count(connection_maker: &impl Connector, change: NewLegionKillCountChange) -> Result<BigDecimal, AdjustmentError> {
        let mut conn = connection_maker.async_connect().await.map_err(AdjustmentError::Connect)?;
        diesel::insert_into(schema::legion_kill_count_changes::table)
            .values(&change)
            .execute(&mut conn)
            .await
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
            .await
            .map_err(AdjustmentError::Count)
    }

    pub async fn count_rows(connection_maker: &impl Connector) -> DbResult<i64> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::legion_kill_counts::table.count().get_result(&mut conn).await?)
    }

    pub async fn get_rank_of(connection_maker: &impl Connector, user_id: UserId) -> DbResult<i64> {
        let mut conn = connection_maker.async_connect().await?;
        let user_record = Self::load_for(connection_maker, user_id).await;
        let kills = user_record.as_ref().map(|r| r.kills.clone()).unwrap_or_default();
        let usage = user_record.as_ref().map(|r| r.updated).unwrap_or(Utc::now() + Duration::milliseconds(100));
        Ok(schema::legion_kill_counts::table
            .filter(
                schema::legion_kill_counts::updated.lt(usage)
                    .and(schema::legion_kill_counts::kills.gt(kills))
            )
            .select(diesel::dsl::count(schema::legion_kill_counts::id))
            .get_result::<i64>(&mut conn)
            .await?)
    }

    pub async fn load_asc(connection_maker: &impl Connector, start: i64, lim: i64) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::legion_kill_counts::table
            .order((schema::legion_kill_counts::kills.desc(), schema::legion_kill_counts::updated))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
            .await?)
    }

    pub async fn load_desc(connection_maker: &impl Connector, start: i64, lim: i64) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::legion_kill_counts::table
            .order((schema::legion_kill_counts::kills, schema::legion_kill_counts::updated.desc()))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
            .await?)
    }

    pub async fn delete(connection_maker: &impl Connector, deleter: UserId, ids: &[BigDecimal]) -> Result<usize, AdjustmentError> {
        let mut conn = connection_maker.async_connect().await.map_err(AdjustmentError::Connect)?;
        let data = diesel::delete(
            schema::legion_kill_counts::table
                .filter(schema::legion_kill_counts::id.eq_any(ids))
        ).get_results::<Self>(&mut conn).await.map_err(AdjustmentError::Change)?;
        let deleted_record_count = data.len();

        // write changes back to db
        diesel::insert_into(schema::legion_kill_count_changes::table)
            .values(data.into_iter().map(|LegionKillCount { id, kills, .. }| NewLegionKillCountChange {
                updater: u64::from(deleter).into(),
                target: id,
                kills,
            }).collect::<Vec<_>>())
            .execute(&mut conn)
            .await
            .map_err(AdjustmentError::Count)?;

        Ok(deleted_record_count)
    }
}
