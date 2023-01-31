mod dbus;
use dbus::service;

use std::error::Error;

use tokio;
use tokio::sync::mpsc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    service::setup_server().await?;

    let (tx, mut rx) = mpsc::channel(uuid::Uuid::new_v4().to_u128_le() % usize::MAX);

    loop {
        std::future::pending::<()>().await;
    }
    
    Ok(())
}

