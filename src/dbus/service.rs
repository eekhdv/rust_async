use zbus::ConnectionBuilder;

use super::notifications::Notifications;


pub async fn setup_server() -> Result<(), Box<dyn std::error::Error>> {
    let _ = ConnectionBuilder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at("/org/freedesktop/Notifications", Notifications)?
        .build()
        .await?;
    Ok(())
}
