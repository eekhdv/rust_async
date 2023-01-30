use zbus::ConnectionBuilder;

use super::notifications::Notifications;


pub async fn setup_server() -> Result<(), Box<dyn std::error::Error>> {
    let srvc_name = "org.freedesktop.Notifications";
    let srvc_obj = "/org/freedesktop/Notifications";

    let _ = ConnectionBuilder::session()?
        .name(srvc_name)?
        .serve_at(srvc_obj, Notifications)?
        .build()
        .await?;
    Ok(())
}
