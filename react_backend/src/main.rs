mod init;
mod models;

use init::configure_mongo::sync_configuration;
#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    sync_configuration().await;

    Ok(())
}
