use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use sqlx_conditional_queries::conditional_query_as;
use std::{env, str::FromStr};

use crate::{IPGetParams, SortBy};

pub struct DB {
    pool: SqlitePool,
}

#[derive(Debug)]
pub enum IPError {
    NotFound,
    Database(sqlx::Error),
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct IPData {
    pub id: i64,
    pub ping_avg: Option<f64>,
    pub down_avg: Option<i64>,
    pub up_avg: Option<i64>,
    pub total_avg: Option<f64>,
    pub ip: String,
    pub cf_country: Option<String>,
    pub ip_country: Option<String>,
}

impl DB {
    pub async fn new() -> tide::Result<Self> {
        let db_url: String =
            std::env::var("DATABASE_URL").expect("Expected Database URL in env vars");
        let pool = SqlitePool::connect(&db_url).await?;
        Ok(Self { pool })
    }

    pub async fn get(
        &self,
        IPGetParams {
            country,
            limit,
            sort,
        }: IPGetParams,
    ) -> Result<Vec<IPData>, IPError> {
        let country_obj = celes::Country::from_str(country.unwrap_or("".to_string()).as_str()).ok();
        let country = country_obj.map(|c| c.alpha2);
        let sort = sort.unwrap_or(SortBy::Ping);
        let rows = conditional_query_as!(
            IPData,
            r#"
                SELECT * 
                FROM ips
                {#filter}
                ORDER BY {#sort}
                LIMIT {#limit}
            "#,
            #filter = match  country.clone() {
                Some(_) =>  "WHERE cf_country = {country}",
                None => "",
            },
            #sort = match sort  {
                SortBy::Ping => "ping_avg ASC",
                SortBy::Speed => "total_avg DESC",
                SortBy::Random => "RANDOM()",
            },
            #limit = match limit {
                Some(_) =>  "{limit}" ,
                None => "10",
            },
        )
        .fetch_all(&self.pool)
        .await;

        match rows {
            Ok(rows) => Ok(rows),
            Err(sqlx::Error::RowNotFound) => Err(IPError::NotFound),
            Err(e) => Err(IPError::Database(e)),
        }
    }
}
