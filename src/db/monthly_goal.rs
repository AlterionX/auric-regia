use std::collections::HashMap;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::{BoolExpressionMethods, DecoratableTarget, ExpressionMethods, OptionalExtension, QueryDsl, prelude::{AsChangeset, Identifiable, Insertable, Queryable}};
use diesel_async::RunQueryDsl;
use crate::schema;

use azel::db::{Connector, DbResult};

#[derive(Debug, Clone)]
#[derive(Insertable)]
#[diesel(table_name = schema::monthly_goals)]
pub struct NewMonthlyGoal<'a> {
    pub updater: BigDecimal,
    pub tag: &'a str,
    pub header: Option<&'a str>,
    pub body: Option<&'a str>,
    pub progress: Option<i16>,
    pub shortname: &'a str,
}

#[derive(Debug, Clone)]
#[derive(Queryable, AsChangeset)]
#[diesel(table_name = schema::monthly_goals)]
pub struct MonthlyGoalUpdate<'a> {
    pub header: Option<&'a str>,
    pub body: Option<&'a str>,
    pub progress: Option<i16>,
}

#[derive(Debug, Clone)]
#[derive(Queryable, Identifiable, Insertable)]
#[diesel(table_name = schema::monthly_goals)]
pub struct MonthlyGoal {
    pub id: i64,
    pub created: DateTime<Utc>,
    pub updater: BigDecimal,
    pub tag: String,
    pub header: String,
    pub body: String,
    pub progress: i16,
    pub shortname: String,
    pub disabled: Option<DateTime<Utc>>,
}

impl MonthlyGoal {
    pub async fn upsert(connection_maker: &impl Connector, new: NewMonthlyGoal<'_>) -> DbResult<()> {
        let mut conn = connection_maker.async_connect().await?;

        diesel::insert_into(schema::monthly_goals::table)
            .values(NewMonthlyGoal {
                header: Some(new.header.unwrap_or("placeholder title")),
                body: Some(new.body.unwrap_or("placeholder body")),
                ..new.clone()
            })
            .on_conflict(schema::monthly_goals::shortname)
            .filter_target(schema::monthly_goals::disabled.is_null())
            .do_update()
            .set(&MonthlyGoalUpdate {
                header: new.header,
                body: new.body,
                progress: new.progress,
            })
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn load_for(connection_maker: &impl Connector, shortname: &str) -> DbResult<Option<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::monthly_goals::table
            .filter(schema::monthly_goals::shortname.eq(shortname))
            .order(schema::monthly_goals::created.desc())
            .limit(1)
            .get_result(&mut conn)
            .await
            .optional()?)
    }

    pub async fn clear_active(connection_maker: &impl Connector) -> DbResult<()> {
        let mut conn = connection_maker.async_connect().await?;

        diesel::update(schema::monthly_goals::table)
            .set(schema::monthly_goals::disabled.eq(diesel::dsl::now))
            .filter(schema::monthly_goals::disabled.is_null())
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn clear_active_by_shortname(connection_maker: &impl Connector, shortname: &str) -> DbResult<()> {
        let mut conn = connection_maker.async_connect().await?;

        diesel::update(schema::monthly_goals::table)
            .set(schema::monthly_goals::disabled.eq(diesel::dsl::now))
            .filter(schema::monthly_goals::disabled.is_null())
            .filter(schema::monthly_goals::shortname.eq(shortname))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn clear_active_by_tag(connection_maker: &impl Connector, tag: &str) -> DbResult<()> {
        let mut conn = connection_maker.async_connect().await?;

        diesel::update(schema::monthly_goals::table)
            .set(schema::monthly_goals::disabled.eq(diesel::dsl::now))
            .filter(schema::monthly_goals::disabled.is_null())
            .filter(schema::monthly_goals::tag.eq(tag))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn load_primary_summary(connection_maker: &impl Connector) -> DbResult<HashMap<String, (i64, i64)>> {
        let mut conn = connection_maker.async_connect().await?;

        let results: Vec<_> = schema::monthly_goals::table
            .group_by(schema::monthly_goals::tag)
            .filter(schema::monthly_goals::disabled.is_null())
            .select((
                schema::monthly_goals::tag,
                diesel::dsl::sum(schema::monthly_goals::progress),
                diesel::dsl::count_star(),
            ))
            .get_results::<(String, Option<i64>, i64)>(&mut conn)
            .await?;

        Ok(results.into_iter().map(|(tag, progress, count)| (tag, (progress.unwrap_or(0), count))).collect())
    }

    pub async fn load_detailed_summary(connection_maker: &impl Connector, tag: &str) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;

        let results: Vec<_> = schema::monthly_goals::table
            .filter(
                schema::monthly_goals::disabled.is_null()
                .and(schema::monthly_goals::tag.eq(tag))
            )
            .get_results(&mut conn)
            .await?;

        Ok(results)
    }

    pub async fn load_active_for_branch(connection_maker: &impl Connector, branch: &str) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;

        Ok(schema::monthly_goals::table
            .filter(schema::monthly_goals::tag.eq(branch))
            .filter(schema::monthly_goals::disabled.is_null())
            .order_by((
                schema::monthly_goals::tag,
                schema::monthly_goals::shortname
            ))
            .get_results(&mut conn)
            .await?)
    }

    pub async fn load_all_active(connection_maker: &impl Connector) -> DbResult<Vec<Self>> {
        let mut conn = connection_maker.async_connect().await?;

        Ok(schema::monthly_goals::table
            .filter(schema::monthly_goals::disabled.is_null())
            .order_by((
                schema::monthly_goals::tag,
                schema::monthly_goals::shortname
            ))
            .get_results(&mut conn).await?)
    }
}
