use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, prelude::{Identifiable, Insertable, Queryable}};
use diesel_async::RunQueryDsl;
use crate::schema;

use azel::db::{Connector, DbResult};

#[derive(Debug, Clone)]
#[derive(Insertable)]
#[diesel(table_name = schema::monthly_goals)]
pub struct NewMonthlyGoal<'a> {
    pub updater: BigDecimal,
    pub tag: &'a str,
    pub header: &'a str,
    pub body: &'a str,
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
}

impl MonthlyGoal {
    pub async fn create(connection_maker: &impl Connector, new: NewMonthlyGoal<'_>) -> DbResult<()> {
        let mut conn = connection_maker.async_connect().await?;

        diesel::insert_into(schema::monthly_goals::table)
            .values(new)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn load_for(connection_maker: &impl Connector, tag: &str) -> DbResult<Option<Self>> {
        let mut conn = connection_maker.async_connect().await?;
        Ok(schema::monthly_goals::table
            .filter(schema::monthly_goals::tag.eq(tag))
            .order(schema::monthly_goals::created.desc())
            .limit(1)
            .get_result(&mut conn)
            .await
            .optional()?)
    }
}
