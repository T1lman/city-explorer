use tokio;
use utils;

#[tokio::main]
async fn main() {
    utils::ui::main::main().await.unwrap();
}
