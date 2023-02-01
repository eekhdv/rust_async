pub enum DbusChannel {
    Notify { notification: Notification },
}

#[derive(Debug)]
pub struct Notification {
    pub app_name: String,
    pub app_icon: String,
    pub title: String,
    pub body: String,
    pub expire_timeout: i32,
}

