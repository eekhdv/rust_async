use std::sync::Arc;
use tokio::sync::Mutex;

use crate::dbus::prep_notifications::Notification;

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

