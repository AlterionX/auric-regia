use std::ops::Add;

use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use diesel::{BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, prelude::{Identifiable, Insertable, Queryable}};
use diesel_async::RunQueryDsl;
use serenity::all::{GuildId, UserId};
use crate::schema;

use azel::db::{Connector, DbResult};

#[derive(Debug, Clone)]
#[derive(Insertable)]
#[diesel(table_name = schema::legion_kill_count_changes)]
pub struct NewLegionKillCountChange {
    pub updater: BigDecimal,
    pub target: BigDecimal,
    pub kills: BigDecimal,
    pub guild_id: BigDecimal,
}

#[derive(Debug, Clone)]
#[derive(Insertable, Queryable, Identifiable)]
#[diesel(table_name = schema::legion_kill_counts)]
pub struct LegionKillCount {
    pub user_id: BigDecimal,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub kills: BigDecimal,
    pub guild_id: BigDecimal,
    pub id: i64,
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
    pub async fn load_for(connection_maker: &impl Connector, user_id: UserId, guild_id: GuildId) -> Option<Self> {
        let mut conn = connection_maker.async_connect().await.ok()?;
        schema::legion_kill_counts::table
            .filter(schema::legion_kill_counts::user_id.eq(BigDecimal::from(u64::from(user_id))))
            .filter(schema::legion_kill_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
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
                schema::legion_kill_counts::guild_id.eq(change.guild_id),
                schema::legion_kill_counts::user_id.eq(change.target),
                schema::legion_kill_counts::updated.eq(diesel::dsl::now),
                schema::legion_kill_counts::kills.eq(max2(
                    BigDecimal::from(0),
                    &change.kills,
                )),
            ))
            .on_conflict((schema::legion_kill_counts::user_id, schema::legion_kill_counts::guild_id))
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

    pub async fn count_rows(connection_maker: &impl Connector, guild_id: GuildId) -> DbResult<i64> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::legion_kill_counts::table.count()
            .filter(schema::legion_kill_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .get_result(&mut conn).await?)
    }

    pub async fn get_rank_of(connection_maker: &impl Connector, user_id: UserId, guild_id: GuildId) -> DbResult<i64> {
        let mut conn = connection_maker.async_connect().await?;
        let user_record = Self::load_for(connection_maker, user_id, guild_id).await;
        let kills = user_record.as_ref().map(|r| r.kills.clone()).unwrap_or_default();
        let usage = user_record.as_ref().map(|r| r.updated).unwrap_or(Utc::now() + Duration::milliseconds(100));
        Ok(schema::legion_kill_counts::table
            .filter(schema::legion_kill_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .filter(
                schema::legion_kill_counts::updated.lt(usage)
                    .and(schema::legion_kill_counts::kills.gt(kills))
            )
            .select(diesel::dsl::count(schema::legion_kill_counts::id))
            .get_result::<i64>(&mut conn)
            .await?)
    }

    pub async fn load_asc(connection_maker: &impl Connector, guild_id: GuildId, start: i64, lim: i64) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::legion_kill_counts::table
            .filter(schema::legion_kill_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .order((schema::legion_kill_counts::kills.desc(), schema::legion_kill_counts::updated))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
            .await?)
    }

    pub async fn load_desc(connection_maker: &impl Connector, guild_id: GuildId, start: i64, lim: i64) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::legion_kill_counts::table
            .filter(schema::legion_kill_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .order((schema::legion_kill_counts::kills, schema::legion_kill_counts::updated.desc()))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
            .await?)
    }

    pub async fn delete(connection_maker: &impl Connector, deleter: UserId, guild_id: GuildId, ids: &[BigDecimal]) -> Result<usize, AdjustmentError> {
        let mut conn = connection_maker.async_connect().await.map_err(AdjustmentError::Connect)?;
        let data = diesel::delete(
            schema::legion_kill_counts::table
                .filter(schema::legion_kill_counts::user_id.eq(BigDecimal::from(u64::from(guild_id))))
                .filter(schema::legion_kill_counts::user_id.eq_any(ids))
        ).get_results::<Self>(&mut conn).await.map_err(AdjustmentError::Change)?;
        let deleted_record_count = data.len();

        // write changes back to db
        diesel::insert_into(schema::legion_kill_count_changes::table)
            .values(data.into_iter().map(|LegionKillCount { user_id, guild_id, kills, .. }| NewLegionKillCountChange {
                updater: u64::from(deleter).into(),
                target: user_id,
                kills,
                guild_id,
            }).collect::<Vec<_>>())
            .execute(&mut conn)
            .await
            .map_err(AdjustmentError::Count)?;

        Ok(deleted_record_count)
    }
}
