use std::sync::Arc;

use tokio::sync::Mutex;

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

#[derive(Debug, Clone)]
pub struct NotificationsDrawer {
    pub notification_boxes: Arc<Mutex<Vec<Notification>>>,
}

impl NotificationsDrawer {
    pub fn new() -> Self {
        NotificationsDrawer {
            notification_boxes: Arc::new(Mutex::new(vec![])),
        }
    }
}

pub struct ScreenDimensions {
    pub width: u32,
    pub height: u32,
}

impl ScreenDimensions {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            width: w,
            height: h,
        }
    }
}
