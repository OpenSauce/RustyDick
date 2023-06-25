use anyhow::Result;
use bot::core::start;

mod bot;
mod commands;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    start().await
}
