use std::error::Error;
use sqlx::mssql::MssqlPool;
use sqlx::{Mssql, Pool};

pub async fn get_mssql_connection() -> Result<Pool<Mssql>, Box<dyn Error>> {
    let pool = MssqlPool::connect("mssql://sa:SqlServer2019!@localhost:1433/master").await?;
    Ok(pool)
}