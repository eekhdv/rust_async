#[derive(Debug)]
pub enum DbusChannel {
    Notify { notification: Notification },
    CloseNotification { unique_id: u32 },
}

#[derive(Debug)]
pub struct Notification {
    pub app_name: String,
    pub app_icon: String,
    pub title: String,
    pub body: String,
    pub expire_timeout: i32,
    pub unique_id: u32,
}

pub fn set_notif_lifetime(id: u32, millis: u64) {
    tokio::task::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(millis))
            .await;
        let connection = zbus::Connection::session().await.unwrap();
        let _ = connection
            .call_method(
                Some("org.freedesktop.Notifications"),
                "/org/freedesktop/Notifications",
                Some("org.freedesktop.Notifications"),
                "CloseNotification",
                &(id),
            )
            .await
            .unwrap();
    });
}
