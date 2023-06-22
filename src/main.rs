mod bot;
mod commands;
mod util;

use bot::core::start;

#[tokio::main]
async fn main() {
    start().await;
}
