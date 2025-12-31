use std::ops::Add;

use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use diesel::{BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, prelude::{Identifiable, Insertable, Queryable}};
use diesel_async::RunQueryDsl;
use serenity::all::{GuildId, UserId};
use crate::schema;

use azel::db::{Connector, DbResult};

#[derive(Debug, PartialEq)]
pub enum AdjustmentError {
    Connect(diesel::result::ConnectionError),
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
    pub guild_id: BigDecimal,
}

#[derive(Debug, Clone)]
#[derive(Insertable, Queryable, Identifiable)]
#[diesel(table_name = schema::naval_victory_counts)]
pub struct NavalVictoryCount {
    pub user_id: BigDecimal,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub victory_fourths: BigDecimal,
    pub guild_id: BigDecimal,
    pub id: i64,
}

diesel::define_sql_function! {
    #[sql_name = "GREATEST"]
    fn max2(a: diesel::sql_types::Numeric, b: diesel::sql_types::Numeric) -> diesel::sql_types::Numeric;
}

impl NavalVictoryCount {
    pub async fn load_for(connection_maker: &impl Connector, user_id: UserId, guild_id: GuildId) -> Option<Self> {
        let mut conn = connection_maker.async_connect().await.ok()?;
        schema::naval_victory_counts::table
            .filter(schema::naval_victory_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .filter(schema::naval_victory_counts::user_id.eq(BigDecimal::from(u64::from(user_id))))
            .get_result(&mut conn)
            .await
            .optional()
            .expect("query to be fine")
    }

    pub async fn adjust_count(connection_maker: &impl Connector, change: NewNavalVictoryCountChange) -> Result<BigDecimal, AdjustmentError> {
        let mut conn = connection_maker.async_connect().await.map_err(AdjustmentError::Connect)?;
        diesel::insert_into(schema::naval_victory_count_changes::table)
            .values(&change)
            .execute(&mut conn)
            .await
            .map_err(AdjustmentError::Change)?;
        diesel::insert_into(schema::naval_victory_counts::table)
            .values((
                schema::naval_victory_counts::guild_id.eq(change.guild_id),
                schema::naval_victory_counts::user_id.eq(change.target),
                schema::naval_victory_counts::updated.eq(diesel::dsl::now),
                schema::naval_victory_counts::victory_fourths.eq(max2(
                    BigDecimal::from(0),
                    &change.victory_fourths,
                )),
            ))
            .on_conflict((schema::naval_victory_counts::user_id, schema::naval_victory_counts::guild_id))
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
            .await
            .map_err(AdjustmentError::Count)
    }

    pub async fn count_rows(connection_maker: &impl Connector, guild_id: GuildId) -> DbResult<i64> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::naval_victory_counts::table.filter(schema::naval_victory_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id)))).count().get_result(&mut conn).await?)
    }

    pub async fn get_rank_of(connection_maker: &impl Connector, user_id: UserId, guild_id: GuildId) -> DbResult<i64> {
        let mut conn = connection_maker.async_connect().await?;
        let user_record = Self::load_for(connection_maker, user_id, guild_id).await;
        let victory_fourths = user_record.as_ref().map(|r| r.victory_fourths.clone()).unwrap_or_default();
        let usage = user_record.as_ref().map(|r| r.updated).unwrap_or(Utc::now() + Duration::milliseconds(100));
        Ok(schema::naval_victory_counts::table
            .filter(schema::naval_victory_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .filter(
                schema::naval_victory_counts::updated.lt(usage)
                    .and(schema::naval_victory_counts::victory_fourths.gt(victory_fourths))
            )
            .select(diesel::dsl::count(schema::naval_victory_counts::id))
            .get_result::<i64>(&mut conn)
            .await?)
    }

    pub async fn load_asc(connection_maker: &impl Connector, guild_id: GuildId, start: i64, lim: i64) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::naval_victory_counts::table
            .filter(schema::naval_victory_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .order((schema::naval_victory_counts::victory_fourths.desc(), schema::naval_victory_counts::updated))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
            .await?)
    }

    pub async fn load_desc(connection_maker: &impl Connector, guild_id: GuildId, start: i64, lim: i64) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::naval_victory_counts::table
            .filter(schema::naval_victory_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .order((schema::naval_victory_counts::victory_fourths, schema::naval_victory_counts::updated.desc()))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
            .await?)
    }

    pub async fn delete(connection_maker: &impl Connector, deleter: UserId, ids: &[i64]) -> Result<usize, AdjustmentError> {
        let mut conn = connection_maker.async_connect().await.map_err(AdjustmentError::Connect)?;
        let data = diesel::delete(
            schema::naval_victory_counts::table
                .filter(schema::naval_victory_counts::id.eq_any(ids))
        ).get_results::<Self>(&mut conn).await.map_err(AdjustmentError::Change)?;
        let deleted_record_count = data.len();

        // write changes back to db
        diesel::insert_into(schema::naval_victory_count_changes::table)
            .values(data.into_iter().map(|NavalVictoryCount { user_id, guild_id, victory_fourths, .. }| NewNavalVictoryCountChange {
                updater: u64::from(deleter).into(),
                target: user_id,
                victory_fourths,
                guild_id,
            }).collect::<Vec<_>>())
            .execute(&mut conn)
            .await
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
    pub guild_id: BigDecimal,
}

#[derive(Debug, Clone)]
#[derive(Insertable, Queryable, Identifiable)]
#[diesel(table_name = schema::naval_tackle_assist_counts)]
pub struct NavalTackleAssistCount {
    pub user_id: BigDecimal,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub tackle_assists: BigDecimal,
    pub guild_id: BigDecimal,
    pub id: i64,
}


impl NavalTackleAssistCount {
    pub async fn load_for(connection_maker: &impl Connector, user_id: UserId, guild_id: GuildId) -> Option<Self> {
        let mut conn = connection_maker.async_connect().await.ok()?;
        schema::naval_tackle_assist_counts::table
            .filter(schema::naval_tackle_assist_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .filter(schema::naval_tackle_assist_counts::user_id.eq(BigDecimal::from(u64::from(user_id))))
            .get_result(&mut conn)
            .await
            .optional()
            .expect("query to be fine")
    }

    pub async fn adjust_count(connection_maker: &impl Connector, change: NewNavalTackleAssistCountChange) -> Result<BigDecimal, AdjustmentError> {
        let mut conn = connection_maker.async_connect().await.map_err(AdjustmentError::Connect)?;
        diesel::insert_into(schema::naval_tackle_assist_count_changes::table)
            .values(&change)
            .execute(&mut conn)
            .await
            .map_err(AdjustmentError::Change)?;
        diesel::insert_into(schema::naval_tackle_assist_counts::table)
            .values((
                schema::naval_tackle_assist_counts::guild_id.eq(change.guild_id),
                schema::naval_tackle_assist_counts::user_id.eq(change.target),
                schema::naval_tackle_assist_counts::updated.eq(diesel::dsl::now),
                schema::naval_tackle_assist_counts::tackle_assists.eq(max2(
                    BigDecimal::from(0),
                    &change.tackle_assists,
                )),
            ))
            .on_conflict((schema::naval_tackle_assist_counts::user_id, schema::naval_tackle_assist_counts::guild_id))
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
            .await
            .map_err(AdjustmentError::Count)
    }

    pub async fn count_rows(connection_maker: &impl Connector, guild_id: GuildId) -> DbResult<i64> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::naval_tackle_assist_counts::table.filter(schema::naval_tackle_assist_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id)))).count().get_result(&mut conn).await?)
    }

    pub async fn get_rank_of(connection_maker: &impl Connector, user_id: UserId, guild_id: GuildId) -> DbResult<i64> {
        let mut conn = connection_maker.async_connect().await?;
        let user_record = Self::load_for(connection_maker, user_id, guild_id).await;
        let tackle_assists = user_record.as_ref().map(|r| r.tackle_assists.clone()).unwrap_or_default();
        let usage = user_record.as_ref().map(|r| r.updated).unwrap_or(Utc::now() + Duration::milliseconds(100));
        Ok(schema::naval_tackle_assist_counts::table
            .filter(schema::naval_tackle_assist_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .filter(
                schema::naval_tackle_assist_counts::updated.lt(usage)
                    .and(schema::naval_tackle_assist_counts::tackle_assists.gt(tackle_assists))
            )
            .select(diesel::dsl::count(schema::naval_tackle_assist_counts::id))
            .get_result::<i64>(&mut conn)
            .await?)
    }

    pub async fn load_asc(connection_maker: &impl Connector, guild_id: GuildId, start: i64, lim: i64) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::naval_tackle_assist_counts::table
            .filter(schema::naval_tackle_assist_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .order((schema::naval_tackle_assist_counts::tackle_assists.desc(), schema::naval_tackle_assist_counts::updated))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
            .await?)
    }

    pub async fn load_desc(connection_maker: &impl Connector, guild_id: GuildId, start: i64, lim: i64) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::naval_tackle_assist_counts::table
            .filter(schema::naval_tackle_assist_counts::guild_id.eq(BigDecimal::from(u64::from(guild_id))))
            .order((schema::naval_tackle_assist_counts::tackle_assists, schema::naval_tackle_assist_counts::updated.desc()))
            .offset(start)
            .limit(lim)
            .get_results(&mut conn)
            .await?)
    }

    pub async fn delete(connection_maker: &impl Connector, deleter: UserId, ids: &[i64]) -> Result<usize, AdjustmentError> {
        let mut conn = connection_maker.async_connect().await.map_err(AdjustmentError::Connect)?;
        let data = diesel::delete(
            schema::naval_tackle_assist_counts::table
                .filter(schema::naval_tackle_assist_counts::id.eq_any(ids))
        ).get_results::<Self>(&mut conn).await.map_err(AdjustmentError::Change)?;
        let deleted_record_count = data.len();

        // write changes back to db
        diesel::insert_into(schema::naval_tackle_assist_count_changes::table)
            .values(data.into_iter().map(|NavalTackleAssistCount { user_id, guild_id, tackle_assists, .. }| NewNavalTackleAssistCountChange {
                updater: u64::from(deleter).into(),
                target: user_id,
                tackle_assists,
                guild_id,
            }).collect::<Vec<_>>())
            .execute(&mut conn)
            .await
            .map_err(AdjustmentError::Count)?;

        Ok(deleted_record_count)
    }
}
