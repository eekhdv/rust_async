use zbus::ConnectionBuilder;

use crate::dbus::raw_handlers::NotificationsHandler;

pub async fn setup_server(
    notification_handler: NotificationsHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let srvc_name = "org.freedesktop.Notifications";
    let srvc_obj = "/org/freedesktop/Notifications";

    let _ = ConnectionBuilder::session()?
        .name(srvc_name)?
        .serve_at(srvc_obj, notification_handler)?
        .build()
        .await?;
    Ok(())
}
