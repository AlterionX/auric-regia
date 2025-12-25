use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, prelude::{AsChangeset, Identifiable, Insertable, Queryable}, query_dsl::methods::{GroupByDsl, SelectDsl}};
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
    pub active: bool,
}

impl MonthlyGoal {
    pub async fn upsert(connection_maker: &impl Connector, new: NewMonthlyGoal<'_>) -> DbResult<()> {
        let mut conn = connection_maker.async_connect().await?;

        diesel::insert_into(schema::monthly_goals::table)
            .values(new.clone())
            .on_conflict(schema::monthly_goals::shortname)
            .do_update()
            // .filter(schema::monthly_goals::active)
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
            .set(schema::monthly_goals::active.eq(false))
            .filter(schema::monthly_goals::active)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn get_summary(connection_maker: &impl Connector, tag: &str) {
        let mut conn = connection_maker.async_connect().await?;

        schema::monthly_goals::table
            .group_by(schema::monthly_goals::tag)
            .filter(shcema::monthly_goals::active)
            .select(schema::monthly_goals::progress)
            .group

        Ok(())
    }
}
