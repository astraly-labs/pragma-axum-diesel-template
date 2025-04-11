mod cli;

use crate::cli::AuthCli;
use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;
use pragma_common::telemetry::init_telemetry;

use pragma_api::{ApiService, AppState};
use pragma_common::services::Service;
use pragma_db::{init_pool, run_migrations};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let AuthCli {
        otel_collector_endpoint,
        database_url,
        api_port,
    } = AuthCli::parse();

    let app_name = "pragma_auth".to_string();
    if let Err(e) = init_telemetry(app_name.clone(), otel_collector_endpoint) {
        panic!("Could not init telemetry: {e}");
    }

    let pool = init_pool(&app_name, &database_url)?;
    run_migrations(&pool).await;

    let app_state = AppState { pool };

    let api_service = ApiService::new(app_state, "0.0.0.0", api_port);
    api_service.start_and_drive_to_end().await?;

    Ok(())
}
