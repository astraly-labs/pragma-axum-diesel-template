use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AuthCli {
    /// Database URL
    #[arg(long, env = "DATABASE_URL")]
    pub database_url: String,

    /// OTEL collector endpoint
    #[arg(long, env = "OTEL_COLLECTOR_ENDPOINT")]
    pub otel_collector_endpoint: Option<String>,

    /// API port
    #[arg(long, env = "API_PORT", default_value = "8080")]
    pub api_port: u16,
}
