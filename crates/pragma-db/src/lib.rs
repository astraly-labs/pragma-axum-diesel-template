use deadpool_diesel::postgres::{Manager, Pool};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use crate::errors::ErrorKind;

pub mod errors;
pub mod models;
pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");

pub async fn run_migrations(pool: &Pool) {
    let conn = pool.get().await.unwrap();
    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();
}

const ENV_DATABASE_MAX_CONN: &str = "DATABASE_MAX_CONN";

pub fn init_pool(app_name: &str, database_url: &str) -> Result<Pool, ErrorKind> {
    let database_max_conn = std::env::var(ENV_DATABASE_MAX_CONN)
        .map_err(|_| ErrorKind::Variable(ENV_DATABASE_MAX_CONN.to_string()))?
        .parse::<u32>()
        .map_err(|_| ErrorKind::GenericInit(format!("cannot parse {ENV_DATABASE_MAX_CONN}")))?
        as usize;

    let manager = Manager::new(
        format!("{database_url}?application_name={app_name}"),
        deadpool_diesel::Runtime::Tokio1,
    );

    Pool::builder(manager)
        .max_size(database_max_conn)
        .build()
        .map_err(|e| ErrorKind::Pool(e.to_string()))
}

#[allow(dead_code)]
type DieselResult<T> = Result<T, diesel::result::Error>;
