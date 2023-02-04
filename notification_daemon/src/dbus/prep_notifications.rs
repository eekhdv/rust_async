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

