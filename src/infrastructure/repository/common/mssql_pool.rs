use sqlx::{database, Mssql, Pool};
use once_cell::sync::OnceCell;
use std::{env, sync::Arc};

pub struct SqlServerPool {
    pool: Arc<Pool<Mssql>>,
}

// Static instance for singleton
static INSTANCE: OnceCell<SqlServerPool> = OnceCell::new();

impl SqlServerPool {
    pub async fn get_instance() -> Result<Arc<Pool<Mssql>>, sqlx::Error> {
         if INSTANCE.get().is_none() {
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file");
            let pool = Pool::<Mssql>::connect(&database_url).await;
            match pool {
                Ok(_) => {
                    let sql_server_pool = SqlServerPool {
                        pool: Arc::new(pool.unwrap()),
                    };
                    let _ = INSTANCE.set(sql_server_pool);
                },
                Err(e) => 
                    return Err(e)
            }
        }

        Ok(INSTANCE.get().unwrap().pool.clone())
    }
}