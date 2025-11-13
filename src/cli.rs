use clap::Parser;

/// Waits for internet connectivity by polling an endpoint
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Maximum number of connection attempts
    #[arg(short, long, default_value_t = 100)]
    pub max_retries: u32,

    /// Endpoint URL or IP to check connectivity
    #[arg(short, long, default_value = "9.9.9.9")]
    pub endpoint: String,

    /// Delay in milliseconds between retry attempts
    #[arg(short, long, default_value_t = 100)]
    pub delay: u64,

    /// Duration in milliseconds before the endpoint is timed out
    #[arg(short, long, default_value_t = 400)]
    pub timeout: u64,
}
